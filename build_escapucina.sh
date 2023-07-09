cd ~/Code/escapucina/yew
trunk build --release --public-url escapucina
ssh raspy rm -r /var/www/escapucina
scp -r dist/* raspy:/var/www/escapucina
ssh raspy chmod 755 /var/www/escapucina
