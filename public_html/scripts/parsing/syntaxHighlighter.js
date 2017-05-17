
var finalString = "";

parserDelegate = function(node)
{
    console.log(node);
}


var config = {sourceType : "script", range : true, loc : true, tokens : true};

//pushes tokens onto the stack and then makes a string at the end 
//this method was used because substring is way faster than replace()
//because it doesn't do a deep copy
//which doesn't matter because we build a new string to return
function highlightSyntax(programText) 
{
    var parsed = esprima.parse(programText, config);
    var stringStack = "";
    var token = parsed.tokens;
    var funcStack;
    //console.log(token);
    if (token[0].value === "function")
    {
        
        for (var i = 0; i < token.length-1; i++)
        {
            funcStack+= token[i].value + " ";
            if (token[i].value === ")") 
            {
                token.splice(0,i+1);
                break; 
            }
        }
    }else
    {
        console.log("function declaration not first or unreadable");
    }
    stringStack += '<div class="code">';//opens the code div
    for (var i = 0; i < token.length-1; i++)
    {
        stringStack += '<pre id="' + token[i].type + '">' + token[i].value;
        for(var sp = token[i].range[1]; sp < token[i+1].range[0]; sp++)
        { 
            if(programText[sp]==="\n"){ stringStack += "<br>"; }
            else{stringStack += '&nbsp'; } 
        }
        stringStack += '</pre>';
    }
    stringStack += '<pre id="' + token[token.length-1].type + '">' + token[token.length-1].value + '</pre>';
    stringStack += '</div>';//closes the code div
    return [stringStack,funcStack];
}




