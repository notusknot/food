var _pyfunc_float = Number;
var _pyfunc_str = String;
var _pyfunc_zip = function () { // nargs: 2 3 4 5 6 7 8 9
    var i, j, tup, arg, args = [], res = [], len = 1e20;
    for (i=0; i<arguments.length; i++) {
        arg = arguments[i];
        if ((typeof arg==="object") && (!Array.isArray(arg))) {arg = Object.keys(arg);}
        args.push(arg);
        len = Math.min(len, arg.length);
    }
    for (j=0; j<len; j++) {
        tup = []
        for (i=0; i<args.length; i++) {tup.push(args[i][j]);}
        res.push(tup);
    }
    return res;
};
var _pymeth_get = function (key, d) { // nargs: 1 2
    if (this.constructor !== Object) return this.get.apply(this, arguments);
    if (this[key] !== undefined) {return this[key];}
    else if (d !== undefined) {return d;}
    else {return null;}
};
var _pymeth_strip = function (chars) { // nargs: 0 1
    if (this.constructor !== String) return this.strip.apply(this, arguments);
    chars = (chars === undefined) ? ' \t\r\n' : chars;
    var i, s1 = this, s2 = '', s3 = '';
    for (i=0; i<s1.length; i++) {
        if (chars.indexOf(s1[i]) < 0) {s2 = s1.slice(i); break;}
    } for (i=s2.length-1; i>=0; i--) {
        if (chars.indexOf(s2[i]) < 0) {s3 = s2.slice(0, i+1); break;}
    } return s3;
};

function defNutrients () {
    var carbsDict, dictsList, fatDict, fiberDict, food, kcalDict, loop, name, nutrient, nutrientList, nutrient_dict, proteinDict, saltDict, saturatesDict, stub1_, stub2_seq, stub3_itr, stub4_tgt, sugarsDict, x;
    // Returns dictionaries for each nutrient, with each food and its respective content
    stub1_ = [({}), ({}), ({}), ({}), ({}), ({}), ({}), ({})];
    kcalDict = stub1_[0];fatDict = stub1_[1];saturatesDict = stub1_[2];carbsDict = stub1_[3];sugarsDict = stub1_[4];proteinDict = stub1_[5];fiberDict = stub1_[6];saltDict = stub1_[7];
    nutrientList = ["kcal", "fat", "saturates", "carbs", "sugars", "protein", "fibre", "salt"];
    dictsList = [kcalDict, fatDict, saturatesDict, carbsDict, sugarsDict, proteinDict, fiberDict, saltDict];
    food = [({name: "Cauliflower & macaroni cheese", author: "Good Food team", description: "What do you get if you add cauliflower, cheese and macaroni? Comfort food heaven", nutrition: ({kcal: "446", fat: "17 g", saturates: "10 g", carbs: "57 g", sugars: "7 g", fibre: "2 g", protein: "19 g", salt: "0.68 g"}), ingredients: ["250g macaroni", "1 head cauliflower", "25g butter", "2 tbsp plain flour", "2 tsp English mustard", "450ml milk", "100g cheddar"], method: ["Cook the macaroni following pack instructions, adding the cauliflower for the final 4 mins.", "Melt the butter in a pan, then stir in the flour and mustard powder and cook for 2 mins. Gradually add the milk, stirring all the time to get a smooth sauce. Add three-quarters of the cheese and some seasoning to the sauce.", "Drain the macaroni and cauliflower and stir into the cheese sauce. Transfer to an ovenproof dish, then sprinkle over the remaining cheese and flash under a hot grill until golden and bubbling. Serve with a green salad, if you like."], time: ({prep: ({hrs: 0, mins: 10}), cook: ({hrs: 0, mins: 20})}), difficulty: "Easy", servings: 4, img_url: "https://images.immediate.co.uk/production/volatile/sites/30/2020/08/recipe-image-legacy-id-775475_10-a73d57a.jpg"}), ({name: "Bean & pesto mash", author: "Good Food team", description: "Substitute potatoes with pulses for a healthy alternative mash with a chunky texture", nutrition: ({kcal: "183", fat: "5 g", saturates: "1 g", carbs: "25 g", sugars: "3 g", fibre: "7 g", protein: "11 g", salt: "0.84 g"}), ingredients: ["1 tbsp olive oil", "2 x 400g cans cannellini beans", "2 tbsp pesto"], method: ["Heat the oil in a large saucepan. Add the beans and cook for 3-4 mins until hot through. Lightly mash with a potato masher for a chunky texture. Stir through the pesto and season. To serve, drizzle with a little olive oil, if you like."], time: ({prep: ({hrs: 0, mins: 5}), cook: ({hrs: 0, mins: 5})}), difficulty: "Easy", servings: 4, img_url: "https://images.immediate.co.uk/production/volatile/sites/30/2020/08/recipe-image-legacy-id-801529_10-82ca74e.jpg"}), ({name: "Brandy pudding", author: "Good Food team", description: "This pud is impressive, delicious and makes a great light alternative dessert to a traditional Christmas pudding", nutrition: ({kcal: "764", fat: "42 g", saturates: "23 g", carbs: "84 g", sugars: "74 g", fibre: "3 g", protein: "6 g", salt: "0.8 g"}), ingredients: ["250g block stoned date", "125g butter", "85g light muscovado sugar", "140g self-raising flour", "1 tsp baking powder", "2 large eggs", "50g raisin", "50g chopped candied peel", "50g dark glac\u00e9 cherry", "50g pecan", "50g stem ginger", "100g caster sugar", "100ml brandy", "butterscotch sauce", "1 holly"], method: ["Heat oven to 180C/160C fan/gas 4. Put the dates in a bowl, pour over 250ml boiling water and set aside. Butter a 1.5-litre pudding basin and put a disc of baking parchment in the base.", "Put the butter, sugar, flour, baking powder and eggs in a large bowl, then beat with an electric hand whisk until well blended. Add the cooled date mix and beat again to make a sloppy batter", "Stir in the fruits, pecans and ginger, then pour into the pudding basin and cover the basin with pleated baking paper and foil (see tips, below). Put the basin in a roasting tin half-filled with boiling water, then steam in the oven for 3 hrs until a skewer inserted into the pudding comes out clean.", "Meanwhile, make the brandy syrup. Put the sugar and 75ml water in a pan and stir over the heat until syrupy. Stir in the brandy and remove from the heat. When the pudding is ready, remove from the oven, take off the foil and paper, and push a skewer into the centre &ndash; it should come out clean. Pour over the brandy syrup so it soaks into the pudding, then cover until ready to serve. Will keep in the fridge for a week.", "To serve, warm the pudding in the turned-off oven once you have taken out the potatoes, or microwave on High, covered with cling film, for 5 mins. Warm the butterscotch sauce (see related recipes) in a pan or in the microwave on Medium for 2 mins. Turn out the pudding onto a plate, pour over some of the sauce and serve the rest in a jug. Top with the holly sprig and serve with good-quality vanilla ice cream"], time: ({prep: ({hrs: 0, mins: 45}), cook: ({hrs: 3, mins: 0})}), difficulty: "More effort", servings: 8, img_url: "https://images.immediate.co.uk/production/volatile/sites/30/2020/08/recipe-image-legacy-id-849513_10-93c1a66.jpg"}), ({name: "Crunchy cauliflower, apple & blue cheese salad", author: "Good Food team", description: "A tasty combination of crunchy cauliflower, sweet apple and creamy, salty blue cheese", nutrition: ({kcal: "417", fat: "30 g", saturates: "13 g", carbs: "18 g", sugars: "15 g", fibre: "5 g", protein: "19 g", salt: "1.05 g"}), ingredients: ["5 tbsp cider vinegar", "2 tbsp extra-virgin olive oil", "little drizzle of honey", "3-4 apple", "1 small cauliflower", "handful of alfalfa", "200g stilton", "small bunch mint", "50g sunflower seeds"], method: ["Make the dressing by whisking the vinegar, olive or rapeseed oil and honey with some seasoning. Pour over the sliced apple then gently mix in the cauliflower, alfalfa sprouts and cheese. Scatter over the mint and the toasted seeds before serving."], time: ({prep: ({hrs: 0, mins: 0}), cook: ({hrs: 0, mins: 15})}), difficulty: "Easy", servings: 4, img_url: "https://images.immediate.co.uk/production/volatile/sites/30/2020/08/recipe-image-legacy-id-824452_10-9c539fd.jpg"}), ({name: "Sticky glazed ribs", author: "James Martin", description: "Treat your guests to these gloriously sticky ribs for a finger-licking lunch", nutrition: ({kcal: "604", fat: "32 g", saturates: "12 g", carbs: "34 g", sugars: "30 g", fibre: "0 g", protein: "48 g", salt: "4.43 g"}), ingredients: ["2kg spare ribs", "2 tbsp barbecue spice", "150g tomato ketchup", "140g chilli ketchup", "3 tbsp soy sauce", "140g runny honey", "3 tbsp teriyaki sauce", "4 tbsp bourbon"], method: ["Heat oven to 160C/140C fan/ gas 3. Place the ribs in a large bowl, then toss with the spice mix so they&rsquo;re covered all over. Place a wire rack in a roasting pan and arrange the ribs in a layer, then cook for 1 hour, until browned and tender.", "Meanwhile, to make the glaze, place the ketchups, soy sauce, honey, teriyaki and whiskey, if using, into a pan, stir well and bring the mixture to a simmer. Simmer for 5 mins until thickened and sticky, then remove from the heat.", "When the ribs are done, remove from the oven and increase the heat to 200C/180C fan/ gas 6. Using a pair of tongs, dip each rib in the glaze, then return to the rack. Place back in the oven and cook for 10 mins. Remove from oven, dip into the glaze again, then return to the oven for another 10-12 mins until sticky. Serve hot with the autumn slaw (see below), chips if you want, and a big pile of napkins."], time: ({prep: ({hrs: 0, mins: 5}), cook: ({hrs: 1, mins: 30})}), difficulty: "Easy", servings: 4, img_url: "https://images.immediate.co.uk/production/volatile/sites/30/2020/08/recipe-image-legacy-id-776589_10-9e10047.jpg"})];
    stub2_seq = _pyfunc_zip(nutrientList, dictsList);
    if ((typeof stub2_seq === "object") && (!Array.isArray(stub2_seq))) { stub2_seq = Object.keys(stub2_seq);}
    for (stub3_itr = 0; stub3_itr < stub2_seq.length; stub3_itr += 1) {
        stub4_tgt = stub2_seq[stub3_itr];
        nutrient = stub4_tgt[0]; nutrient_dict = stub4_tgt[1];
        for (loop = 0; loop < food.length; loop += 1) {
            x = _pymeth_get.call(_pymeth_get.call(food[loop], "nutrition"), nutrient);
            if ((x === null)) {
                x = 0;
            }
            x = _pyfunc_float((_pymeth_strip.call(_pyfunc_str(x), "g")));
            name = _pymeth_get.call(food[loop], "name");
            nutrient_dict[name] = x;
        }
    }
    return dictsList;
};
 var _pyfunc_int = function (x, base) { // nargs: 1 2
    if(base !== undefined) return parseInt(x, base);
    return x<0 ? Math.ceil(x): Math.floor(x);
};
var _pyfunc_op_add = function (a, b) { // nargs: 2
    if (Array.isArray(a) && Array.isArray(b)) {
        return a.concat(b);
    } return a + b;
};
var _pyfunc_op_contains = function op_contains (a, b) { // nargs: 2
    if (b == null) {
    } else if (Array.isArray(b)) {
        for (var i=0; i<b.length; i++) {if (_pyfunc_op_equals(a, b[i]))
                                           return true;}
        return false;
    } else if (b.constructor === Object) {
        for (var k in b) {if (a == k) return true;}
        return false;
    } else if (b.constructor == String) {
        return b.indexOf(a) >= 0;
    } var e = Error('Not a container: ' + b); e.name='TypeError'; throw e;
};
var _pyfunc_op_equals = function op_equals (a, b) { // nargs: 2
    var a_type = typeof a;
    // If a (or b actually) is of type string, number or boolean, we don't need
    // to do all the other type checking below.
    if (a_type === "string" || a_type === "boolean" || a_type === "number") {
        return a == b;
    }

    if (a == null || b == null) {
    } else if (Array.isArray(a) && Array.isArray(b)) {
        var i = 0, iseq = a.length == b.length;
        while (iseq && i < a.length) {iseq = op_equals(a[i], b[i]); i+=1;}
        return iseq;
    } else if (a.constructor === Object && b.constructor === Object) {
        var akeys = Object.keys(a), bkeys = Object.keys(b);
        akeys.sort(); bkeys.sort();
        var i=0, k, iseq = op_equals(akeys, bkeys);
        while (iseq && i < akeys.length)
            {k=akeys[i]; iseq = op_equals(a[k], b[k]); i+=1;}
        return iseq;
    } return a == b;
};
var _pyfunc_sum = function (x) {  // nargs: 1
    return x.reduce(function(a, b) {return a + b;});
};
var _pyfunc_truthy = function (v) {
    if (v === null || typeof v !== "object") {return v;}
    else if (v.length !== undefined) {return v.length ? v : false;}
    else if (v.byteLength !== undefined) {return v.byteLength ? v : false;}
    else if (v.constructor !== Object) {return true;}
    else {return Object.getOwnPropertyNames(v).length ? v : false;}
};
var _pymeth_append = function (x) { // nargs: 1
    if (!Array.isArray(this)) return this.append.apply(this, arguments);
    this.push(x);
};
var _pymeth_items = function () { // nargs: 0
    if (this.constructor !== Object) return this.items.apply(this, arguments);
    var key, keys = Object.keys(this), res = []
    for (var i=0; i<keys.length; i++) {key = keys[i]; res.push([key, this[key]]);}
    return res;
};

function getPermutations(array, size) {

    function p(t, i) {
        if (t.length === size) {
            result.push(t);
            return;
        }
        if (i + 1 > array.length) {
            return;
        }
        p(t.concat(array[i]), i + 1);
        p(t, i + 1);
    }

    var result = [];
    p([], 0);
    return result;
}


function matchBounds(lowerBound, upperBound, nutrientDict, mealAmnt, totalDays) {
    var chosen, comboTuple, j, loop, matchedList, req, stub1_seq, stub2_itr, stub3_seq, stub4_itr, uniqueCombos;
    // Matches a users requirements as an upper and lower bound to items who's sum match it
    matchedList = [];
    for (req = lowerBound; req < upperBound; req += 1) {
        chosen = [];
        uniqueCombos = [];
        j = [];
        stub3_seq = getPermutations(_pymeth_items.call(nutrientDict), mealAmnt);
        console.log(_pymeth_items.call(nutrientDict))
        if ((typeof stub3_seq === "object") && (!Array.isArray(stub3_seq))) { stub3_seq = Object.keys(stub3_seq);}
        for (stub4_itr = 0; stub4_itr < stub3_seq.length; stub4_itr += 1) {
            comboTuple = stub3_seq[stub4_itr];
            stub1_seq = comboTuple;
            if ((typeof stub1_seq === "object") && (!Array.isArray(stub1_seq))) { stub1_seq = Object.keys(stub1_seq);}
            for (stub2_itr = 0; stub2_itr < stub1_seq.length; stub2_itr += 1) {
                loop = stub1_seq[stub2_itr];
                if (_pyfunc_truthy(((!_pyfunc_op_contains(loop, uniqueCombos))) && ((!_pyfunc_op_contains(loop, chosen))))) {
                    _pymeth_append.call(uniqueCombos, loop);
                    _pymeth_append.call(j, _pyfunc_int(loop[1]));
                }
            }
            if ((uniqueCombos.length < mealAmnt)) {
                continue;
            }
            if ((_pyfunc_op_equals(_pyfunc_sum(j), req))) {
                _pymeth_append.call(matchedList, uniqueCombos);
                chosen = _pyfunc_op_add(chosen, uniqueCombos);
                if ((matchedList.length == totalDays)) {
                    console.log(matchedList)
                    return matchedList;
                }
            }
            uniqueCombos = [];
            j = [];
        }
    }
    return matchedList;
};
 var _pyfunc_dict = function (x) {
    var t, i, keys, r={};
    if (Array.isArray(x)) {
        for (i=0; i<x.length; i++) {
            t=x[i]; r[t[0]] = t[1];
        }
    } else {
        keys = Object.keys(x);
        for (i=0; i<keys.length; i++) {
            t=keys[i]; r[t] = x[t];
        }
    }
    return r;
};
var _pyfunc_list = function (x) {
    var r=[];
    if (typeof x==="object" && !Array.isArray(x)) {x = Object.keys(x)}
    for (var i=0; i<x.length; i++) {
        r.push(x[i]);
    }
    return r;
};
var _pymeth_items = function () { // nargs: 0
    if (this.constructor !== Object) return this.items.apply(this, arguments);
    var key, keys = Object.keys(this), res = []
    for (var i=0; i<keys.length; i++) {key = keys[i]; res.push([key, this[key]]);}
    return res;
};

function shuffle(array) {
  let currentIndex = array.length,  randomIndex;

  // While there remain elements to shuffle...
  while (currentIndex != 0) {

    // Pick a remaining element...
    randomIndex = Math.floor(Math.random() * currentIndex);
    currentIndex--;

    // And swap it with the current element.
    [array[currentIndex], array[randomIndex]] = [
      array[randomIndex], array[currentIndex]];
  }

  return array;
}

var l, n, nutrientDict, shuffledDict;
n = _pymeth_items.call(((defNutrients()[0])));
l = _pyfunc_list(n);
shuffle(l);
shuffledDict = _pyfunc_dict(l);
nutrientDict = _pyfunc_dict(_pymeth_items.call(shuffledDict));

console.log(defNutrients())
console.log(matchBounds(0, 2200, nutrientDict, 3, 7))

