# ovpn-linux-auto-dns

 Automatically update resolv.conf on linux systems when using OVPN (open VPN) connection.

If you are sucessfully able to connect to an open vpn connection but you are not able to resolve some of the private urls for your org the likely reason for this is that there is a private DNS instance which holds these zone files which are not being used by your system. On windows and MAC the open VPN client will automatically update your DNS settings but this does not seem to work as expected on linux clients.

Previously I was using the project https://github.com/jonathanio/update-systemd-resolved but this has a dependency on resolvconf svc which is not used in many distributions like Fedora and Debian and attempting to install it can have unexpected results. I was looking for a more simple solution that updated resolv.conf directly. This application will parse the std out of the openvpn connection, search for DNS servers and take the first match, then it will check your /etc/resolv.conf for this entry and if it is not available it will be added as the primary DNS server.

This application will not remove the record after the VPN connection is stopped, in my experiance these will be removed on restart or when connection is reset. Connection reset and restart will also disconnect VPN so it does not seem that at least in my configuration this feature is required. 

This requires the open vpn command line interface, I have made no attempt to implement this with any connection manager from a specific distro.

# installation

Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Build application

```bash
git clone https://github.com/jasonwitty/ovpn-linux-auto-dns/
cd ovpn-linux-auto-dns
cargo build --release
```

Add path to .bashrc (~/.bashrc) and save.

```bash
export PATH="$PATH:/home/jason/Documents/GitHub/ovpn-linux-auto-dns/target/release/"
```

reload

```bash
source ~/.bashrc
```

# usage

Simply pipe the output of your open vpn connection to this program

```bash
sudo openvpn --config ./any.ovpn --script-security 2 | sudo ovpn-linux-auto-dns
Enter Auth Username: Jason.Witty
Enter Auth Password: ****************
CHALLENGE: Enter Authenticator Code 000000
Found DNS entry: x.x.x.x
Successfully added x.x.x.x to /etc/resolv.conf
Complete, Added : x.x.x.x to resolv.conf. exiting...
```
