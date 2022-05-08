import json
import itertools
import random
from pprint import pprint

def matchBounds(boundsList, nutrientDict, mealAmnt: int, totalDays: int):
    lowerBound = boundsList[0][0]
    upperBound = boundsList[0][1]

    '''Matches a users requirements as an upper and lower bound to items who's sum match it'''
    # List of combinations
    matchedList = [] 
    for req in range(lowerBound, upperBound):
        # All the food you chose
        chosen = []

        # Unique food for the day 
        uniqueCombos = []

        # Values for the foods
        valueList = []
        for comboTuple in itertools.combinations(nutrientDict, mealAmnt):
            for loop in comboTuple:
                if loop not in uniqueCombos and loop not in chosen:
                    uniqueCombos.append(loop)
                    valueList.append(int(loop[1]))

            if len(uniqueCombos) < mealAmnt:
                continue

            if sum(valueList) == req:
                matchedList.append(uniqueCombos)
                chosen += uniqueCombos
                if len(matchedList) == totalDays:
                    return matchedList

            uniqueCombos = []
            valueList = []

def defNutrients():
    nutrient_data = { "kcal": {}, "fat": {}, "saturates": {}, "carbs": {}, "sugars": {}, "protein": {}, "fibre": {}, "salt": {}, }

    with open ("./food.json", "r") as jsonFile:
        foodJson = json.load(jsonFile)

    for food in foodJson:
        for nutrient in nutrient_data:
            nutrient_data[nutrient][food["name"]] = float(str(food["nutrition"].get(nutrient, "0") or "0").strip("g"))

    return nutrient_data

def prepNutrients(nutrientDict, nutrient):
    n = list(nutrientDict[nutrient].items())
    random.shuffle(n)

    return n

if __name__ == '__main__':
    bounds = [ (2000, 2200, "kcal"), (100, 120, "protein"), ]
        
    nutrientDict = prepNutrients(defNutrients(), bounds[0][2])

    pprint(matchBounds(bounds, nutrientDict, 3, 7))

    #shuffleNutrients(defNutrients(), bounds[0][2])
