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
$ export TIKV_VERSION=v7.5.0
$ export GOOS=darwin  # only {darwin, linux} are supported
$ export GOARCH=amd64 # only {amd64, arm64} are supported
$ curl -O  https://tiup-mirrors.pingcap.com/tikv-$TIKV_VERSION-$GOOS-$GOARCH.tar.gz
$ curl -O  https://tiup-mirrors.pingcap.com/pd-$TIKV_VERSION-$GOOS-$GOARCH.tar.gz
$ tar -xzf tikv-$TIKV_VERSION-$GOOS-$GOARCH.tar.gz
$ tar -xzf pd-$TIKV_VERSION-$GOOS-$GOARCH.tar.gz
```

2. Start PD instance.

```bash
$ ./pd-server --name=pd --data-dir=/tmp/pd/data --client-urls="http://127.0.0.1:2379" --peer-urls="http://127.0.0.1:2380" --initial-cluster="pd=http://127.0.0.1:2380" --log-file=/tmp/pd/log/pd.log
```

3. Start TiKV instance.

```bash
$ ./tikv-server --pd-endpoints="127.0.0.1:2379" --addr="127.0.0.1:20160" --data-dir=/tmp/tikv/data --log-file=/tmp/tikv/log/tikv.log
```

4. Install TiKV Client(Python) and verify the deployment, required Python 3.5+.

```bash
$ pip3 install -i https://test.pypi.org/simple/ tikv-client
```
