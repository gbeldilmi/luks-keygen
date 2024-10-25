# luks-keygen

This is a pseudo-random key-file generator to encrypt partitions with LUKS.

## Available options

- `-l <length>`: The length of the keyfile to be generated. Default is 512 characters.
- `-c <charset>`: The charset to be used to generate the keyfile. Default is from `0x21` to `0x7E` (ASCII printable characters).
- `-o <output>`: The output file to save the keyfile. Default is `key.txt`.
