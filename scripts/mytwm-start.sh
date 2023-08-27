#!/bin/sh

# genome-keyring
if [[ -z "${DBUS_SESSION_BUS_ADDRESS}" ]];
then
    eval $(dbus-launch --sh-syntax --exit-with-session)
fi
# Make the keyring daemon ready to communicate with nm-applet
export $(gnome-keyring-daemon --start --components=pkcs11,secrets,ssh,gpg)
xrdb -merge $HOME/.Xresources
xrandr --setprovideroutputsource modesetting NVIDIA-0
xrandr --auto

while true; do
    mytwm &> /home/taru/work/mytwm-home/.mytwm.log
    [[ $? > 0 ]] && mv /home/taru/work/mytwm-home/.mytwm.log /home/taru/work/mytwm-home/.prev-mytwm.log
    export RESTARTED=true
done
