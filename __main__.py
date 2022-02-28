import json

def matchBounds(lowerBound: int, upperBound: int, numList: list):
    for req in range(lowerBound, upperBound):
        n2 = req//2
        answers = {req-x for x in numList if x <= n2} & {x for x in numList if x>n2}
        for x in answers:
            foodList.append((req-x, x))

foodList = []
if __name__ == '__main__':
    # Define nutrients
    kcalDict, fatDict, saturatesDict, carbsDict, sugarsDict, proteinDict, fiberDict, saltDict = {}, {}, {}, {}, {}, {}, {}, {}
    nutrientList = ["kcal", "fat", "saturates", "carbs", "sugars", "protein", "fibre", "salt"]
    dicts_list = [kcalDict, fatDict, saturatesDict, carbsDict, sugarsDict, proteinDict, fiberDict, saltDict]

    with open ("food.json", "r") as jsonFile:
        food = json.load(jsonFile)

    for nutrient, nutrient_dict in zip(nutrientList, dicts_list):
        for loop in range(0,len(food)):
            x = food[loop].get("nutrition").get(nutrient)
            name = food[loop].get("name")

            nutrient_dict[name] = x 

    print(saltDict)
