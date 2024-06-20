# ipcount

Expand the CIDR notation into a list of IP addresses, or inversely, unite the
list of IP addresses into a CIDR notation.

# Usage

```
$ echo -n "10.0.2.254\n10.0.2.255\n10.0.3.0\n10.0.3.1\n" | cargo run -q --bin ipunite
10.0.2.254/31
10.0.3.0/31
```

```
$ echo -n "10.0.2.254/31\n10.0.3.0/31\n" | cargo run -q --bin ipcount
10.0.2.254
10.0.2.255
10.0.3.0
10.0.3.1
```
