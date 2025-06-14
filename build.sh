#!/bin/bash

# Build script for Z language compiler

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Building Z language compiler...${NC}"

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed.${NC}"
    echo -e "Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if GCC is installed (we use it instead of LLVM now)
if ! command -v gcc &> /dev/null; then
    echo -e "${YELLOW}Warning: GCC not found. Attempting to install...${NC}"
    
    # Try to install GCC based on the OS
    if command -v apt-get &> /dev/null; then
        echo "Installing GCC using apt..."
        sudo apt-get update
        sudo apt-get install -y gcc
    elif command -v brew &> /dev/null; then
        echo "Installing GCC using Homebrew..."
        brew install gcc
    elif command -v pacman &> /dev/null; then
        echo "Installing GCC using pacman..."
        sudo pacman -S gcc
    else
        echo -e "${RED}Error: Could not install GCC automatically.${NC}"
        echo "Please install GCC manually and try again."
        exit 1
    fi
fi

# Build the compiler
echo "Building Z compiler..."
cargo build --release

# Create symbolic links
echo "Creating symbolic links..."
mkdir -p bin
ln -sf ../target/release/zc bin/zc

# Run tests
echo "Running tests..."
cargo test

# Create a simple wrapper script
cat > bin/z << EOF
#!/bin/bash
# Z language wrapper script
SCRIPT_DIR=\$(dirname "\$(readlink -f "\$0")")
\$SCRIPT_DIR/zc "\$@"
EOF

chmod +x bin/z

echo -e "${GREEN}Build completed successfully!${NC}"
echo "You can now use the Z compiler with: ./bin/zc or ./bin/z"
echo ""
echo "Examples:"
echo "  ./bin/z run examples/test.z     # Run a Z program"
echo "  ./bin/z compile examples/test.z # Compile a Z program to executable"