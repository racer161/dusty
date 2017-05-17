//initializes the keyboard input for a new func
function initializeListeners(input, column, lineBox, errorBox)
    {
        var savedSelection;

        //Called everytime text area is touched
        function objUpdate() 
        {

            column.innerHTML = window.getSelection().anchorOffset;
            try{
                errorBox.innerText = "";
                var result = highlightSyntax(input.innerText);
                input.innerHTML = result[0];
                
            }catch(err){
                //console.log(err);
                errorBox.innerText = JSON.stringify(err);
            }

        }


        //LineBox Handler
        function addLineBox (len)
        {
            lineBox.innerHTML = "";
            lineBox.insertAdjacentHTML("beforeend" ,"<p class='lineNumber'>0</p>");
            for (var i = 1; i < len; i++)
            {
                lineBox.insertAdjacentHTML("beforeend","<p class='lineNumber'>" + i + "</p>"); 
            }
        }

        //handles all the key strokes
        function keyHandler(e){
            var code = e.keyCode;
            //console.log(e.key);
            if (code === 9) 
            { // tab key
                e.preventDefault();  // this will prevent us from tabbing out of the editor

                var sel = window.getSelection();
                // now insert four non-breaking spaces for the tab key
                var range = sel.getRangeAt(0);

                var tabNode = document.createTextNode("\u00a0\u00a0\u00a0\u00a0");
                range.insertNode(tabNode);

                range.setStartAfter(tabNode);
                range.setEndAfter(tabNode);



                sel.removeAllRanges();
                sel.addRange(range);

                savedSelection = saveSelection(input);

                objUpdate();

                onSubtreeModified();

                return false;
            }else if((code > 36 && code < 41) || (code >7 && code < 21))
            {
                //DO NOTHING
                addLineBox(stringtools.findNewLineCount(input.innerText));
            }
            else{
                e.preventDefault();  // this will prevent us from tabbing out of the editor
                
                var sel = window.getSelection();
                
                var range = sel.getRangeAt(0);
                var textNode = document.createTextNode(e.key);
                range.insertNode(textNode);

                range.setStartAfter(textNode);
                range.setEndAfter(textNode);


                //restoreSelection(input, savedSelection);
                sel.removeAllRanges();
                sel.addRange(range);

                savedSelection = saveSelection(input);

                objUpdate();

                onSubtreeModified();
                return false;
            }

        }

        //restores mouse position after the 
        function onSubtreeModified ()
        {
            restoreSelection(input, savedSelection);
            addLineBox(stringtools.findNewLineCount(input.innerText));
        }

        input.onkeydown = keyHandler;
    }