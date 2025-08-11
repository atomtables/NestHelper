# NestHelper NestMap

NestMap being a map of all the important things that NestHelper requires.

## `nest caddy list`
lists all domains currently added to Nest.
```
- <username>.hackclub.app (unix//home/atomtables/.<username>.hackclub.app.webserver.sock)
- sub.domain.tld (unix//home/atomtables/.sub.domain.tld.webserver.sock)
`````
should only return zero.

## `nest caddy add <domain>`
adds a domain to Nest.

success (code 0):
```
chat.atomtables.dev added. (unix//home/atomtables/.chat.atomtables.dev.webserver.sock)
```
failure (code that is not 0):
```
The domain `<domain>` is not verified.

There are two ways to verify your domain:

- Add a TXT record to your domain (google.com) to "domain-verification=atomtables". You can remove it after it is added.
- Set the CNAME record on your domain (google.com) to `atomtables.hackclub.app`.

If you have already done this, please wait a few minutes for DNS records to propagate.
```
on failure just end execution and send the STDOUT.

## `nest caddy rm <domain>`
removes a domain from Nest.
success (code 0):
```
chat.atomtables.dev removed.
```
failure (code that is not 0): no one geniunely cares just print stdouterr or crap.

## `cat ~/Caddyfile'
caddyfile always exists. if it doesn't just touch Caddyfile and return empty.

also caddyfile needs a caddyfile parser just because.

## wait... `caddy adapt` (dtm tho)
converts caddyfile to json
