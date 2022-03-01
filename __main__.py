import json
import itertools

def matchBounds(lowerBound: int, upperBound: int, nutrientDict: dict, mealAmnt: int):
    '''Matches a users requirements as an upper and lower bound to items who's sum match it'''
    nutrientValList = list(map(int, list(nutrientDict.values())))
    matchedList = []
    for req in range(lowerBound, upperBound):
        for i in itertools.combinations(nutrientValList, mealAmnt):
            if sum(i) == req:
                matchedList.append(i)

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
            x = float(str(x).strip(" g"))
            name = food[loop].get("name")

            nutrient_dict[name] = x
    return dictsList 

# TODO print name of food with value

if __name__ == '__main__':
    kcalValList = list(map(int, list(defNutrients()[0].values())))
    kcalNameList = list(defNutrients()[0].keys())
    kcalList = list(defNutrients()[0].keys())
    kcalDict = defNutrients()[0]
    kcalDict = dict(itertools.islice(kcalDict.items(), 2))
    
    print(matchBounds(int(input("Lower bound: ")), int(input("Upper bound: ")), kcalDict, 3))
