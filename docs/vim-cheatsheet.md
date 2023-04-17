# VIM NOTES

[Customize VIM](https://www.freecodecamp.org/news/vimrc-configuration-guide-customize-your-vim-editor/)
[Customize NeoVim](https://www.linode.com/docs/guides/how-to-install-neovim-and-plugins-with-vim-plug/)
[Vim-Plug](https://github.com/junegunn/vim-plug)

zo to open a single fold under the cursor.

zc to close the fold under the cursor.

zR to open all folds.

zM to close all folds.


## Install a Vim plugin manager

Let's install the vim plugin manager - vim-plug
```
curl -fLo ~/.vim/autoload/plug.vim --create-dirs \
    https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
```
    
For neovim the config dir is: ~/.config/nvim/init.vim


To show what has been read by VIM:
```
:scriptnames
```

Let's install a color scheme

```
cd ~/.vim/colors
curl -o molokai.vim https://raw.githubusercontent.com/tomasr/molokai/master/colors/molokai.vim
```
:colorscheme molokai


