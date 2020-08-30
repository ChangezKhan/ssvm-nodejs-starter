const express = require('express');
const fileUpload = require('express-fileupload');
const { rgb_to_hex } = require('../pkg/ssvm_nodejs_starter_lib.js');

const app = express();
const port = 3000;
app.use(express.static(__dirname + '/public'));
app.use(fileUpload());

app.get('/', (req, res) => res.redirect("/index.html"));

app.post('/convert_to_hex', function (req, res) {
  console.log(req.body.red, req.body.green, req.body.blue);
  res.send(rgb_to_hex(req.body.red, req.body.green, req.body.blue));
})

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))
