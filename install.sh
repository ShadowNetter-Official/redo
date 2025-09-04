#!/bin/bash

echo
echo "redo"
echo "by ShadowNetter"
echo
echo "cloning into repo..."
git clone https://github.com/ShadowNetter-Official/redo
cd redo
echo "done"
echo "installing..."
chmod +x redo
sudo cp redo ~/.cargo/bin/
mkdir ~/.config/redo
touch ~/.config/redo/tasks.json
echo "done"
echo
echo "to uninstall do: sudo rm /bin/redo"
echo
