import json
from collections.abc import MutableMapping

def flatten(d,sep='_'):
    import collections

    obj = collections.OrderedDict()

    def recurse(t,parent_key=''):
        
        if isinstance(t,list):
            for i in range(len(t)):
                recurse(t[i],parent_key)
        elif isinstance(t,dict):
            for k,v in t.items():
                recurse(v,k)
        else:
            obj[parent_key] = t

    recurse(d)

    return obj

text = 'use crate::lib::FoodItem;\nlet: &[FoodItem] = &['

with open('food.json', 'r') as read_file:
    data = json.load(read_file)
    for loop in list(data):
        loop = dict(flatten(loop))
        if loop['kcal'] != None and loop['fat'] != None and loop['saturates'] != None and loop['carbs'] != None and loop['sugars'] != None and loop['fibre'] != None and loop['protein'] != None and loop['salt'] != None:
            text += '\nFoodItem{ name: "'+loop['name']+'", description: "'+loop['description']+'", author: "'+loop['author']+'", ingredients: "'+loop['ingredients']+'", instructions: "'+loop['method']+'", difficulty: "'+loop['difficulty']+'", img_url: "'+loop['img_url']+'", servings: '+str(loop['servings']).split(".")[0].strip(".")+', kcal: '+str(loop['kcal']).split(".")[0].strip(".")+', fat: '+str(loop['fat']).strip(' g').split(".")[0].strip(".")+', saturates: '+str(loop['saturates']).strip(' g').split(".")[0].strip(".")+', carbs: '+str(loop['carbs']).strip(' g').split(".")[0].strip(".")+', sugars: '+str(loop['sugars']).strip(' g').split(".")[0].strip(".")+', fiber: '+str(loop['fibre']).strip(' g').split(".")[0].strip(".")+', protein: '+str(loop['protein']).strip(' g').split(".")[0].strip(".")+', salt: '+str((loop['salt']).strip(' g')).split(".")[0].strip(".")+', },'
        #name, author, description, nutrition, kcal, fat, saturates, carbs, sugars, fibre, protein, salt, ingredients, method, time, difficulty, servings, img_url
        break

text += '\n];'
print(text)