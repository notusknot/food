import json
import itertools
import random
from pprint import pprint
import time


function matchBounds(lowerBound, upperBound, nutrientObj, mealAmnt, totalDays) {
    nutrientValList = Object.values(nutrientObj)
    nutrientKeyList = Object.keys(nutrientObj)
    matchedList = []
    for (let req = lowerBound; i < upperBound; i++){
        for (let comboTuple in )
    }
}

def matchBounds(lowerBound: int, upperBound: int, nutrientDict: dict, mealAmnt: int, totalDays: int):
    #nutrientValList = list(map(int, list(nutrientDict.values())))
    nutrientValList = list(nutrientDict.values())
    nutrientKeyList = list(nutrientDict.keys())
    matchedList = []

    for req in range(lowerBound, upperBound):
        for comboTuple, j in zip(itertools.combinations(nutrientDict.items(), mealAmnt), itertools.combinations(nutrientValList, mealAmnt)):
            for loop in comboTuple:
                print(loop)
                nutrientDict.pop(loop[0], None)

            if sum(j) == req:
                matchedList.append(comboTuple)


                if len(matchedList) == totalDays:
                    return matchedList

def defNutrients():
    '''Returns dictionaries for each nutrient, with each food and its respective content'''
    kcalDict, fatDict, saturatesDict, carbsDict, sugarsDict, proteinDict, fiberDict, saltDict = {}, {}, {}, {}, {}, {}, {}, {}
    nutrientList = ["kcal", "fat", "saturates", "carbs", "sugars", "protein", "fibre", "salt"]
    dictsList = [kcalDict, fatDict, saturatesDict, carbsDict, sugarsDict, proteinDict, fiberDict, saltDict]

    with open ("food.json", "r", encoding="utf-8") as jsonFile:
        food = json.load(jsonFile)

    for nutrient, nutrient_dict in zip(nutrientList, dictsList):
        for loop in range(len(food)):
            x = food[loop].get("nutrition").get(nutrient)
            if x == None:
                x = 0
            x = float(str(x).strip("g"))
            name = food[loop].get("name")

            nutrient_dict[name] = x
    return dictsList 

if __name__ == '__main__':
    n = defNutrients()[0].items()
    l = list(n)
    random.shuffle(l)
    shuffledDict = dict(l)

    #nutrientDict = dict(itertools.islice(shuffledDict.items(), 20))
    nutrientDict = dict(shuffledDict.items())
    #kcalDict = dict(itertools.islice(defNutrients()[2].items(), 20))
        
    #pprint(matchBounds(int(input("Lower bound: ")), int(input("Upper bound: ")), nutrientDict, int(input("Meals per day: ")), int(input("Total days: "))))
    pprint(matchBounds(2000, 2200, nutrientDict, 3, 7))
