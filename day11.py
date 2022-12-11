# -*- coding: utf-8 -*-
"""
Created on Sun Dec 11 20:25:43 2022

@author: BLTom
"""
import math
import heapq
from day11Inputs import *  


#Number of Monkeys
monkeyNumbers = 8
#Number of items which pass by each monkey
monkeyCount = [0] * monkeyNumbers
#number of rounds to simulate
numRounds =20
i = 0
while numRounds != 0:
    #get current monkey
    monkey = monkeys[i%monkeyNumbers]
    
    for item in monkey["items"]:  
        value = monkey["value"]
        if monkey["value"] == -1:
            value = item
        result = eval(f'{item} {monkey["operation"]} {value}')
        noBreak = math.floor(result/3)
        
        if noBreak%monkey["test"] ==0:
            monkeys[monkey["true"]]["items"].append(noBreak)
        else:
            monkeys[monkey["false"]]["items"].append(noBreak)
            
        monkeyCount[i%monkeyNumbers] +=1
    monkey["items"].clear()
    if i%monkeyNumbers == monkeyNumbers-1:
        numRounds -=1
    i +=1
print(heapq.nlargest(2, monkeyCount))

