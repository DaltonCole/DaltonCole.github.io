/* Notes:
 *  Origin:
 *      * The origin is the top-left corner. This point is (0, 0)
 *  Drawing rectangles:
 *      * Draw a filled rectangle: fillRect(x, y, width, height)
 *      * Draw a rectangular outline: strokeRect(x, y, width, height)
 *      * Make a transparent rectangle: clearRect(x, y, width, height)
 *  Drawing paths:
 *      * A path is a list of points, connected by segments of which can be different shapes, i.e. straight vs curved
 *      * Functions:
 *          * beginPath();  // Start a path (does nothing but restarts a path)
 *          * Path functions:
 *              * moveTo(x, y); // Lifts the "pen" from the canvas and moves it to the position
 *              * lineTo(x, y); // Draws a line from current position to (x, y)
 *              * bezierCurveTo(controlPoint1X, controlPoint1Y, controlPoint2X, controlPoint2Y, x, y);
 *              * quadraticCurveTo(controlPointX, controlPointy, x, y);
 *              * arc(x, y, radius, startAngle, endAngle, anticlockwise); // *Angle = points of the arc in radians(radians = MATH.PI/180 * degrees). Anticlockwise = boolean
 *              * arcTo(x1, y1, x2, y2);
 *              * ellipse()
 *              * rect()
 *          * stroke(); // Creates an outline
 *          * fill(); // Fills shape. Auto closePath.
 *          * closePath();  // Go back to start of the current sub-path
 *      * Examples:
 *          * Triangle: ctx.beginPath(); ctx.moveTo(75,75); ctx.lineTo(100, 75); ctx.lineTo(100, 25); ctx.fill();
 *  Colors and Style:
 *      * Colors:
 *          * fillStyle = color; // Fill color
 *          * strokeStyle = color; // Outline color
 *              * color is a CSS color. For example "rgb(255, 0, 0)" or "#FF0000" or "red" for red
 */

function main() {
    var canvas = document.getElementById('snake-canvas'); // Get canvas from DOM
    if(canvas.getContext) {
        var ctx = canvas.getContext('2d'); // Get context, so we can draw

        // Get dimensions
        var width = canvas.width;
        var height = canvas.height;

        console.log("Width:  " + width);
        console.log("Height: " + height);

        ctx.fillStyle = 'rgb(200, 0, 0)';
        ctx.fillRect(10, 10, 50, 10);
    }
}
