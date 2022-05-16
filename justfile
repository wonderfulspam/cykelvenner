# Installs the necessary npm packages
install_frontend:
  cd frontend && npm install

_build_backend:
  cargo build

# Installs all dependencies
install: install_frontend _build_backend

# Builds the application
build:
  cd frontend && npm run build
  cargo build
  
# Runs the application
run:
  cargo run --quiet

@_check_is_installed command url:
  echo -n "Checking {{command}}..."
  command -v {{command}} >/dev/null && echo "OK!" || printf "Failed!\nPlease see <{{url}}>\n"
  

# Checks if necessary development tools are installed
@check_requirements:
  echo 'Checking OS...{{ if os_family() == "windows" { "Windows? Oh no." } else { "OK!" } }}'
  just _check_is_installed npm "https://docs.npmjs.com/downloading-and-installing-node-js-and-npm/"
  just _check_is_installed cargo "https://www.rust-lang.org/tools/install"

