# IP-Checker

A small utility for private usage.

The reasoning is that my ISP does not provide me with a static IP address, and as such I am subject to rather random external IPv4 changes.
This means that anything that I proxy behind cloudflare to reach my home can often fail.

This utility will fetch my current IPv4 address and update my Cloudflare A record for home as needed.

It checks every 5 minutes.
