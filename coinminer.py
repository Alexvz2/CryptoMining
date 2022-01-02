from hashlib import sha256
import time

max_coin_blob = 100000000000000000
time.time()
for coin_blob in range(max_coin_blob):
    string = "CPEN 442 Coin" + "2021"+\
        "a9c1ae3f4fc29d0be9113a42090a5ef9fdef93f5ec4777a008873972e60bb532" + \
        str(coin_blob) + "e08d03262ca4a1992ea9177dad54784d35c0d3736b56ee6bb0d2f28383df673c"
    hashed = sha256(string.encode('ascii')).hexdigest()
    if hashed.startswith("00000000"):
        print("Example Coin Verified!")
        print(coin_blob)
        print(hashed)
        break