# ----------------------------------------------------
# Directory Configuration
# ----------------------------------------------------

DOTILES_DIR="$HOME/dotfiles"
DOTILES_SHELL_DIR="$DOTILES_DIR/shell"

# ----------------------------------------------------
# Shell Configuration
# ----------------------------------------------------
# Load: ZSH Plugins 
source $DOTILES_SHELL_DIR/plugins.zsh

# Load: PATH
export PATH=$DOTILES_DIR/bin:$PATH

# Load: ENV Variables
source $DOTILES_SHELL_DIR/variables.zsh

# Load: Aliases
source $DOTILES_SHELL_DIR/aliases.zsh