// Get DOM elements
const form  = document.querySelector('form');
const input = document.querySelector("[name='todo']");
const todoList = document.getElementById('todos');

// Side Effects / Lifecycle
const existingTodos = JSON.parse(localStorage.getItem('todos')) || [];

const todoData = [];

existingTodos.forEach(todo => {
    addTodo(todo);
});


function addTodo(todoText) {
    todoData.push(todoText);
    const li = document.createElement('li')
    li.innerHTML = todoText;
    todoList.appendChild(li);
    localStorage.setItem('todos', JSON.stringify(todoData));
    input.value = ''
}

function* range(start, end) {
    for (; start <= end; ++start) { yield start; }
}

function last(arr) { return arr[arr.length - 1]; }

function* numericCombinations(n, r, loc = []) {
    const idx = loc.length;
    if (idx === r) {
        yield loc;
        return;
    }
    for (let next of range(idx ? last(loc) + 1 : 0, n - r + idx)) { yield* numericCombinations(n, r, loc.concat(next)); }
}

function* combinations(arr, r) {
    for (let idxs of numericCombinations(arr.length, r)) { yield idxs.map(i => arr[i]); }
}

/*
def combinations (r) {
        let arr = this.toArray(),
            len = toPositiveInteger(r),
            res = [];

        return new Iter(function* gen(idx = 0, start = 0) {
            if (idx >= len) {
                yield res.slice();
                return;
            }
            for (let i = start, l = arr.length; i < l; i++) {
                res[idx] = arr[i];
                yield* gen(idx + 1, i + 1);
            }
        });
    }
*/
function matchBounds(targetNum, nutrientObj, mealAmnt, totalDays) {
/*    matchedList = []
    chosen = []
    uniqueCombos = []
    j = []*/

    for (let combo = 0; combo < combinations(nutrientObj.values(), mealAmnt); combo++ ) {
        console.log(combo)
        console.log(nutrientObj)
        /*for loop in comboTuple:
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
        j = []*/
    }
}


fetch("./food.json")
.then(response => { return response.json(); })

.then(data => console.log(data));

// Events
form.onsubmit = (event) => {
    event.preventDefault();
    addTodo(input.value);
    print(jsonObject)
    /*matchBounds()*/
}


