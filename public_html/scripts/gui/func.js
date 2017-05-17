document.addEventListener("DOMContentLoaded", function (event) {
var funcTree = {};
    
var elementTree = [];

//pushes a new method object onto the local project funcTree
function instantiateFuncFrame(id, programText)
{
    funcTree[id] = ({"id" : id, "text" : programText, "dependencies": [], "dependents": [] });
    
}

//renders the func tree
function renderFuncTree()
{
    var index = 0;
    
    for (func in funcTree)
    {
        var funcIndx = funcTree[func];
        console.log(func);
        var newFuncBlock = createNewFuncBlock(funcIndx.id);
        document.body.appendChild(newFuncBlock);
        document.getElementById("codeText"+funcIndx.id).innerText  = funcIndx.text; initializeListeners(document.getElementById("codeText"+funcIndx.id),document.getElementById("co"+funcIndx.id),document.getElementById("lineBox"+funcIndx.id),document.getElementById("errorBox"+funcIndx.id));
    }
}


function createNewFuncBlock(id)
{
    var funcBlockTemplate = document.createElement("DIV");
    funcBlockTemplate.className +="funcBlock";
    funcBlockTemplate.id = id;
    funcBlockTemplate.innerHTML = '<div class="funcHeader"></div>\
            <div class="codeBox">\
                <div class="codeArea">\
                    <div class="lineBox" id="lineBox'+ id +'">\
                        <p class="lineNumber">0</p>\
                    </div>\
                    <div class="codeAreaText" id="codeText'+ id +'" contenteditable></div>\
                    <div class="errorBox" id="errorBox'+ id +'"> </div>\
                    <div class="lineInfo">\
                        <div class="co">COL: </div>\
                        <div id="co'+ id +'">0</div></div>\
                </div>\
            </div>';
    
    return funcBlockTemplate;
}
    
    
instantiateFuncFrame(0, 'function bubbleSort(a)\r\n{\r\n    var swapped;\r\n    do {\r\n        swapped = false;\r\n        for (var i=0; i < a.length-1; i++) {\r\n            if (a[i] > a[i+1]) {\r\n                var temp = a[i];\r\n                a[i] = a[i+1];\r\n                a[i+1] = temp;\r\n                swapped = true;\r\n            }\r\n        }\r\n    } while (swapped);\r\n}var a = [34, 203, 3, 746, 200, 984, 198, 764, 9];\r\n bubbleSort(a);');
renderFuncTree();  
    
});