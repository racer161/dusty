var stringtools = {
    
    findNewlineIndices : function (str) 
    {
        "use strict";
        var indices = [];
        for (var i = 0; i < str.length; i++) 
        {
            if(str[i] === "\n")
            {
                indices.push(i);
            }
        }
        return indices;
    },
    findNewLineCount : function (str) 
    {
        "use strict";
        var indices = 0;
        for (var i = 0; i < str.length; i++) 
        {
            if(str[i] === "\n")
            {
                indices+=1;
            }
        }
        return indices;
    }
    
    
}