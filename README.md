# Netfilter tracker

Successor of [Gotrtack](https://github.com/untoldwind/gotrack)

## Prepare the router

* The tacker expects your to use `dnsmasq` and support `conntrack` (the `conntrackd` is not required)
* ... otherwise you may setup your router any way you like.
* The tracker requires `conntrack_acct` being enabled. Either with 
  ```
  sysctl -w net.netfilter.nf_conntrack_acct = 1
  ```
  or
  ```
  echo "1" > /proc/sys/net/netfilter/nf_conntrack_acct
  ```
* To enable this at system boot you probably already have a `/etc/sysctl.d/99-ipforward.conf` (or similar)
  ```
  net.netfilter.nf_conntrack_acct=1  
  net.ipv4.ip_forward=1
  net.ipv6.conf.default.forwarding=1
  net.ipv6.conf.all.forwarding=1
  ```
* On some systems this will not work out of the box since the `nf_conntrack` kernel module might be loaded/activated to late during boot. To fix this you can add a `/etc/modules-load.d/nf_conntrack.conf`
  ```
  nf_conntrack
  ```
