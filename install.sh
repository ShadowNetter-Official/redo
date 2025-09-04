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
cp redo ~/.cargo/bin/
mkdir ~/.config/redo
touch ~/.config/redo/tasks.json
echo "done"
echo
echo "to uninstall do: rm ~/.cargo/bin/"
echo
