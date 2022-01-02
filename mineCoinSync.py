import hashlib as hl
import requests as rq
from sys import argv, maxsize
from base64 import b64encode
from json import dumps
from time import time, sleep, asctime

class Miner():
    def __init__(self, id, id_of_miner):
        self.id = id
        self.name = "CPEN 442 Coin2021"
        self.hash_of_preceding_coin = None
        self.coin_blob = ""
        self.id_of_miner = self.SHA256(id_of_miner)
        self.hash_of_current_coin = ""
        self.difficulty = None
        self.current_difficulty = 0
        self.lastUpdated = None
        self.updateCoinVal()
        print("Miner {} initiated".format(self.id))

    def SHA256(self, hash_string):
        return hl.sha256(hash_string.encode()).hexdigest()

    def getCoinHash(self):
        try:
            string = "".join([self.name+self.hash_of_preceding_coin+self.coin_blob+self.id_of_miner])
            return self.SHA256(string)
        except TypeError:
            self.updateCoinVal()
            return self.getCoinHash()

    def claimCoin(self):
        url = "http://cpen442coin.ece.ubc.ca/claim_coin"
        pload = {
            "coin_blob": b64encode(self.coin_blob.encode('ascii')).decode('utf-8'),
            "id_of_miner": self.id_of_miner,
            "hash_of_last_coin" : self.hash_of_preceding_coin
        }
        output = rq.post(url, data=dumps(pload), headers={ "Content-Type": "application/json" })
        print(output)
        return output

    def getLastCoin(self):
        url = "http://cpen442coin.ece.ubc.ca/last_coin"
        output = rq.post(url)
        if output.status_code == 200:
            return output.json()["coin_id"]
        sleep(1)
        return self.getLastCoin() # loop until we get the last coin

    def getDifficulty(self):
        url = "http://cpen442coin.ece.ubc.ca/difficulty"
        output = rq.post(url)
        return int(output.json()["number_of_leading_zeros"])

    def isHashValid(self, difficulty=None):
        if difficulty == None:
            difficulty = self.difficulty
        return self.getCoinHash().startswith("0"*difficulty)

    def updateCoinVal(self):
        last_coin = self.getLastCoin()
        difficulty = self.getDifficulty()
        self.lastUpdated = time()
        if self.hash_of_preceding_coin != last_coin or self.difficulty != difficulty:
            self.hash_of_preceding_coin = last_coin 
            self.difficulty = difficulty
            self.current_difficulty = 0
            return True
        return False

    def mineBlock(self):
        i = 0
        while i <= maxsize:
            self.coin_blob = str(i)
            if time() - self.lastUpdated > 3000: #  check for coin values every 5 mins
                if self.updateCoinVal():
                    print("Coin has been updated {}".format(self.hash_of_preceding_coin))
                    return

            if self.isHashValid():
                output = "Miner #{} found coin blob {} with hash {}".format(self.id, self.coin_blob, self.getCoinHash())
                response = self.claimCoin()
                self.printOutput(output, response)
                return 

            elif self.isHashValid(self.current_difficulty+1):
                self.current_difficulty += 1
                print("Miner #{} at difficulty {}/{}, hash: {} for hash:{}".format(self.id, self.current_difficulty, self.difficulty, self.getCoinHash(), self.hash_of_preceding_coin))
            i += 1

    def printOutput(self, output, response):
        f = open("Problem1Output.txt", "w")
        print("Output: {} \nResponse: {}".format(output, response))
        f.write("Output: {} \nResponse: {}".format(output, response))
        f.close()

    def startLoop(self):
        while True:
            self.mineBlock()
            self.updateCoinVal()

def miner_thread():
    coin = Miner(0,"bush_did_911")
    coin.startLoop()

if __name__ == "__main__":
    print("Minning started {}".format(asctime()))
    miner_thread()
    print(asctime() + " Miner Stopped")
