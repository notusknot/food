# this file is to convert my food.json to a static slice in data.rs
# python3 json_to_rust.py > src/data.rs
# make sure food.json is presentx

import json
from collections.abc import MutableMapping

# flattens a dict
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

text = 'use crate::lib::FoodItem;\npub static NUTRIENTS: &[FoodItem] = &['

with open('food.json', 'r') as read_file:
    data = json.load(read_file)
    for loop in list(data):
        loop = dict(flatten(loop))
        if loop['name'] != None and loop['author'] != None and loop['description'] != None and loop['ingredients'] != None and loop['difficulty'] != None and loop['kcal'] != None and loop['fat'] != None and loop['saturates'] != None and loop['carbs'] != None and loop['sugars'] != None and loop['fibre'] != None and loop['protein'] != None and loop['salt'] != None:
            text += '\nFoodItem{ name: "'+loop['name']+'".to_string(), description: "'+loop['description']+'".to_string(), author: "'+loop['author']+'".to_string(), ingredients: "'+loop['ingredients']+'".to_string(), instructions: "'+loop['method']+'".to_string(), difficulty: "'+loop['difficulty']+'".to_string(), img_url: "'+loop['img_url']+'".to_string(), servings: '+str(loop['servings']).split(".")[0].strip(".")+', kcal: '+str(loop['kcal']).split(".")[0].strip(".")+', fat: '+str(loop['fat']).strip(' g').split(".")[0].strip(".")+', saturates: '+str(loop['saturates']).strip(' g').split(".")[0].strip(".")+', carbs: '+str(loop['carbs']).strip(' g').split(".")[0].strip(".")+', sugars: '+str(loop['sugars']).strip(' g').split(".")[0].strip(".")+', fiber: '+str(loop['fibre']).strip(' g').split(".")[0].strip(".")+', protein: '+str(loop['protein']).strip(' g').split(".")[0].strip(".")+', salt: '+str((loop['salt']).strip(' g')).split(".")[0].strip(".")+', },'
        #name, author, description, nutrition, kcal, fat, saturates, carbs, sugars, fibre, protein, salt, ingredients, method, time, difficulty, servings, img_url
        break

text += '\n];'
print(text)