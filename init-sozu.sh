#!/bin/sh

ctl() {
	docker exec playground_sozu_1 /sozuctl --config /etc/sozu/config.toml "$@"
}

ctl application add --id 'id:ping-1' --load-balancing-policy roundrobin
ctl application add --id 'id:ping-2' --load-balancing-policy roundrobin

ctl backend add --address '172.24.0.3:80' --backend-id 'backend:ping-1' --id 'id:ping-1'
ctl backend add --address '172.24.0.2:80' --backend-id 'backend:ping-2' --id 'id:ping-2'

ctl frontend http add --address '0.0.0.0:80' --hostname 'ping-1.local.rgafiyatullin.me' --id 'id:ping-1'
ctl frontend http add --address '0.0.0.0:80' --hostname 'ping-2.local.rgafiyatullin.me' --id 'id:ping-2'
