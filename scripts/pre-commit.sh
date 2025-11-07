#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Running pre-commit checks...${NC}\n"

# Step 1: Auto-format code
echo -e "${YELLOW}1. Running cargo fmt (auto-fix)...${NC}"
if cargo fmt --all -- --check > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Code is already formatted${NC}\n"
else
    echo -e "${YELLOW}⚠ Formatting code...${NC}"
    cargo fmt --all
    echo -e "${GREEN}✓ Code formatted (files auto-fixed)${NC}\n"
    
    # Stage the formatted files
    git add -u
fi

# Step 2: Run clippy
echo -e "${YELLOW}2. Running cargo clippy...${NC}"
if cargo clippy --all-targets --all-features --workspace -- -D warnings; then
    echo -e "${GREEN}✓ Clippy passed${NC}\n"
else
    echo -e "${RED}✗ Clippy failed${NC}"
    echo -e "${RED}Please fix the clippy warnings before committing${NC}"
    echo -e "${YELLOW}Hint: Run 'cargo clippy --fix --all-targets --all-features --workspace' to auto-fix some issues${NC}"
    exit 1
fi

# Step 3: Run cargo check
echo -e "${YELLOW}3. Running cargo check...${NC}"
if cargo check --all-features --workspace > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Check passed${NC}\n"
else
    echo -e "${RED}✗ Cargo check failed${NC}"
    echo -e "${RED}Please fix compilation errors before committing${NC}"
    exit 1
fi

# Step 4: Run tests (optional - can be commented out for faster commits)
# Uncomment the lines below to run tests on every commit
# echo -e "${YELLOW}4. Running tests...${NC}"
# if cargo test --all-features --workspace > /dev/null 2>&1; then
#     echo -e "${GREEN}✓ Tests passed${NC}\n"
# else
#     echo -e "${RED}✗ Tests failed${NC}"
#     echo -e "${RED}Please fix failing tests before committing${NC}"
#     exit 1
# fi

echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓ All pre-commit checks passed!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
