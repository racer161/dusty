//Gets the raw screen pixels of an element
function getScreenCordinates(obj) {
    "use strict";
    var p = {};
    p.x = obj.offsetLeft;
    p.y = obj.offsetTop;
    while (obj.offsetParent) {
        p.x = p.x + obj.offsetParent.offsetLeft;
        p.y = p.y + obj.offsetParent.offsetTop;
        if (obj === document.getElementsByTagName("body")[0]) {
            break;
        } else {
            obj = obj.offsetParent;
        }
    }
    return p;
}

//dragListener constructor
//the listener should be a div inside the object to "click and drag" by
function dragListener(listener, movedOBJ, maxX, maxY, minX, minY) {
    "use strict";
    var offset, x, y, currentTransform = {x: 0, y: 0};
    
    if (arguments.length <= 2) {
        maxY = window.innerHeight;
        maxX = window.innerWidth;
        minX = 0;
        minY = 0;
    } else if (arguments.length <= 4) {
        maxY = window.innerHeight;
        maxX = window.innerWidth;
    }
    
    function mouseMove(e) {
        x = currentTransform.x + (e.x - offset.x);
        y = currentTransform.y + (e.y - offset.y);
        
        if (x < minX) { x = minX; }
        if (y < minY) { y = minY; }
        if (x > maxX) { x = maxX; }
        if (y > maxY) { y = maxY; }
        
        movedOBJ.style.transform = "translate(" + x + "px, " + y + "px)";
    }
    
    function mouseDown(e) {
        //Setting the limits on drag and drop
        offset = { x : e.x, y : e.y };
        document.addEventListener("mousemove", mouseMove);
        document.addEventListener("mouseup", function (e) {
            document.removeEventListener('mousemove', mouseMove);
            currentTransform.x = x;
            currentTransform.y = y;
        });
    }
        
        
    /*--------------------------TOUCH-------------------------------------*/
        
    function touchMove(e) {
        x = currentTransform.x + (e.x - offset.x);
        y = currentTransform.y + (e.y - offset.y);
        
        if (x < minX) { x = minX; }
        if (y < minY) { y = minY; }
        if (x > maxX) { x = maxX; }
        if (y > maxY) { y = maxY; }
        
        movedOBJ.style.transform = "translate(" + x + "px, " + y + "px)";
    }
    
    function touchDown(e) {
        //Setting the limits on drag and drop
        offset = { x : e.x, y : e.y };
        document.body.addEventListener("touchmove", touchMove);
        document.body.addEventListener("touchup", function (e) {
            document.body.removeEventListener('touchmove', touchMove);
            currentTransform.x = x;
            currentTransform.y = y;
        });
        
    }
    
    listener.addEventListener("mousedown", function (e) { mouseDown(e); });//mouse
    listener.addEventListener("touchdown", function (e) { touchDown(e); });//touch
    
    
    //dragListener(document.getElementById('browserViewTab'), document.getElementById('browserView'), 0, 0, 0, -(input.clientHeight)+(input.clientHeight*.05));
    

}