# PAC for the foboot peripherals

SVD is generated from https://github.com/im-tomu/foboot, with lxsocdoc updated
to the current version:

``` sh
$ cd hw
hw$ cd deps/lxsocdoc
hw/deps/lxsocdoc$ git pull
[..]
hw/deps/lxsocdoc$ cd ../../
hw$ python3 foboot-bitstream.py --document-only
```
You can now replace the current svd file with the one under
`hw/build/software/Fomu.svd` and regenerate the rust code with:

``` sh
$ sh generate.sh
```

You need svd2rust & form for this to work.
