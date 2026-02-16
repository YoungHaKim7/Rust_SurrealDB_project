# tikv-client
- 260216개발중인가 겁나게 에러난다.
- https://github.com/surrealdb/tikv-client/blob/master/README.md
- https://tikv.org/docs/7.1/develop/clients/rust/

# tikv(github)

- Distributed transactional key-value database, originally created to complement TiDB
  - https://github.com/tikv/tikv
- https://tikv.org/

# Install

```bash
❯ curl --proto '=https' --tlsv1.2 -sSf https://tiup-mirrors.pingcap.com/install.sh | sh
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 4404k  100 4404k    0     0  19.5M      0 --:--:-- --:--:-- --:--:-- 19.4M
Successfully set mirror to https://tiup-mirrors.pingcap.com
Detected shell: fish
Shell profile:  /Users/gy-gyoung/.profile
/Users/gy-gyoung/.profile has been modified to add tiup to PATH
open a new terminal or source /Users/gy-gyoung/.profile to use it
Installed path: /Users/gy-gyoung/.tiup/bin/tiup
===============================================
Have a try:     tiup playground
===============================================
```

# Save data folder

- data폴더에 저장되는듯


```bash
~
~/.tiup
❯ ls
bin/        components/ data/       history/    manifests/  storage/    tiup.toml

~/.tiup
❯ eza -la -TL3
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 .
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 ├── bin
.rw-r--r--@ 7.3k gy-gyoung 16 Feb 23:19 │   ├── 7b8e153f2e2d0928.root.json
.rw-r--r--@ 7.3k gy-gyoung 16 Feb 23:21 │   ├── root.json
.rwxr-xr-x@  12M gy-gyoung 16 Feb 23:21 │   └── tiup
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 ├── components
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   ├── grafana
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   │   └── v8.5.5
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   ├── pd
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   │   └── v8.5.5
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   ├── playground
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   │   └── v1.16.4
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   ├── prometheus
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   │   └── v8.5.5
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   └── tikv
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │       └── v8.5.5
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 ├── data
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   ├── Users
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   │   └── gy-gyoung
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   └── VBOpYxt
.rw-r--r--@    0 gy-gyoung 16 Feb 23:21 │       ├── dsn
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │       ├── grafana
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │       ├── pd-0
.rw-r--r--@    4 gy-gyoung 16 Feb 23:21 │       ├── port
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │       ├── prometheus
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │       └── tikv-0
drwxr-xr-x@    - gy-gyoung 16 Feb 23:20 ├── history
.rw-r--r--@  164 gy-gyoung 16 Feb 23:21 │   └── tiup-history-0
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 ├── manifests
.rwxr-xr-x@ 7.3k gy-gyoung 16 Feb 23:21 │   ├── 3.root.json
.rwxr-xr-x@ 7.3k gy-gyoung 16 Feb 23:21 │   ├── 4.root.json
.rwxr-xr-x@ 7.3k gy-gyoung 16 Feb 23:21 │   ├── 5.root.json
.rwxr-xr-x@ 7.3k gy-gyoung 16 Feb 23:21 │   ├── 6.root.json
.rwxr-xr-x@ 207k gy-gyoung 16 Feb 23:21 │   ├── grafana.json
.rwxr-xr-x@ 7.4k gy-gyoung 16 Feb 23:21 │   ├── index.json
.rwxr-xr-x@ 202k gy-gyoung 16 Feb 23:21 │   ├── pd.json
.rwxr-xr-x@ 125k gy-gyoung 16 Feb 23:21 │   ├── playground.json
.rwxr-xr-x@ 210k gy-gyoung 16 Feb 23:21 │   ├── prometheus.json
.rwxr-xr-x@ 7.3k gy-gyoung 16 Feb 23:21 │   ├── root.json
.rwxr-xr-x@ 3.2k gy-gyoung 16 Feb 23:21 │   ├── snapshot.json
.rwxr-xr-x@ 208k gy-gyoung 16 Feb 23:21 │   ├── tidb.json
.rwxr-xr-x@ 205k gy-gyoung 16 Feb 23:21 │   ├── tikv.json
.rwxr-xr-x@  684 gy-gyoung 16 Feb 23:21 │   └── timestamp.json
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 ├── storage
drwxr-xr-x@    - gy-gyoung 16 Feb 23:21 │   └── playground
.rw-r--r--@   44 gy-gyoung 16 Feb 23:21 └── tiup.toml
```
