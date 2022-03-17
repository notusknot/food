import json
import itertools
import random
from pprint import pprint

def matchBounds(boundsList, nutrientDict, mealAmnt: int, totalDays: int):
    lowerBound = boundsList[0][0]
    upperBound = boundsList[0][1]

    '''Matches a users requirements as an upper and lower bound to items who's sum match it'''
    matchedList = []
    for req in range(lowerBound, upperBound):
        # All the food you chose
        chosen = []

        # Unique food for the day 
        uniqueCombos = []
        j = []
        for comboTuple in itertools.combinations(nutrientDict.items(), mealAmnt):
            for loop in comboTuple:
                if loop not in uniqueCombos and loop not in chosen:
                    uniqueCombos.append(loop)
                    j.append(int(loop[1]))

            if len(uniqueCombos) < mealAmnt:
                continue

            if sum(j) == req:
                matchedList.append(uniqueCombos)
                chosen += uniqueCombos
                if len(matchedList) == totalDays:
                    return matchedList

            uniqueCombos = []
            j = []

def defNutrients():
    '''Returns dictionaries for each nutrient, with each food and its respective content'''
    kcalDict, fatDict, saturatesDict, carbsDict, sugarsDict, proteinDict, fiberDict, saltDict = {}, {}, {}, {}, {}, {}, {}, {}
    nutrientList = ["kcal", "fat", "saturates", "carbs", "sugars", "protein", "fibre", "salt"]
    dictsList = [kcalDict, fatDict, saturatesDict, carbsDict, sugarsDict, proteinDict, fiberDict, saltDict]
    dictsDict = {} 

    with open ("./food.json", "r") as jsonFile:
        food = json.load(jsonFile)

    for nutrient, nutrient_dict in zip(nutrientList, dictsList):
        for loop in range(len(food)):
            x = food[loop].get("nutrition").get(nutrient)
            if x is None:
                x = 0
            x = float(str(x).strip("g"))
            name = food[loop].get("name")

            nutrient_dict[name] = x
        dictsDict[nutrient] = nutrient_dict

        

    return dictsDict 

def shuffleNutrients(nutrientDict, nutrient):
    n = nutrientDict[nutrient].items()
    l = list(n)
    random.shuffle(l)
    shuffledDict = dict(l)

    return dict(shuffledDict.items())

if __name__ == '__main__':
    bounds = [
        # kcal
        (100, 120, "protein"),

        # protein
        #(0, 2000, "protein"),

        # carbs
        #(0, 2000, "carbs")
    ]
        
    nutrientDict = shuffleNutrients(defNutrients(), bounds[0][2])

    pprint(matchBounds(bounds, nutrientDict, 3, 7))
    #pprint(matchBounds(2000, 2200, nutrientDict, 3, 7))
