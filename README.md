# truffle-plays

## A new and improved way to have your chat play games!

## How it works

Viewers see a button on their screen with a blue background and black play button on it.
After pressing it they will see a controller with buttons on it.
In order for the input from the buttons to be sent to your computer they need to add the server url by clicking on the chain icon and typing it in.
More on where you get this url from in the how to use section.
Now they can press buttons and send inputs to the server.
It is set that they can send 2 inputs every second.
Rejoice no more waiting to be able to send identical messages again.
In order for them to know what buttons they are pressing they can view the key map in the image below (scroll down a little bit if you don't see it.)
We reccomend having a graphic or command that shows what the buttons do as the keymap will say the keycodes on our end but not what it does on your computer.
These inputs get sent to the server which stores them until your pc reads them.
In order for you to read the inputs there is a client you run on your computer which gets the inputs and presses the key on your computer based on what you have the inputs set to.

## Keymap

https://github.com/eoghanriley/truffle-plays/blob/main/keymap.png

## How to use

### Contact

If you have any issues in setting it up or find any bugs you can either create an issue here on github or email me at truffle@eoghanriley.dev
If you find a security issue ONLY send it truffle@eoghanriley.dev and please do it promptly so it can be fixed.
Lastly if you use this shoot me and email at truffle@eoghanriley.dev so I can see it an action.

### Server

We reccomend https://fly.io as your hosting provider because they have a gracious free period.
In order to deploy the server code hit the big green button that says Code on https://github.com/eoghanriley/truffle-plays then select Download Zip.
Once you have done that extract the zip and open up the fly.toml.
In fly.toml change the value next to AUTH from e to a secure password this can be with notepad or any other text editor.
WARNING this is what gets used to verify that it is you reading the inputs so make it secure and don't give it away.
I will never ask you for it so don't give it away.
Then install flyctl from https://fly.io/docs/hands-on/install-flyctl/.
Open up the extracted folder and go into the server directory and then press ALT+D.
This should open up a new window called the command prompt.
Once the command prompt is open type flyctl launch.
This will walk you through launching the server and will require you to have an account and card on file with them.
Once it says deployed on the fly dashboard the server side is done.

### Streamer-client

Go back to https://github.com/eoghanriley/truffle-plays (if your reading this your probably there already).
Hit the releases button and download the latest streamer-client from there.
This will place it in your downloads folder.
We reccomend creating a new folder either in Downloads or somewhere else to hold this.
Now in the same place the streamer client is create a file called settings.json
Paste this in as a starting point

```
{
  "d1_left": "a",
  "d1_up": "w",
  "d1_right": "d",
  "d1_down": "s",

  "x1": "h",
  "a1": "j",
  "b1": "k",
  "y1": "l",

  "d2_left": "z",
  "d2_up": "x",
  "d2_right": "c",
  "d2_down": "v",

  "x2": "b",
  "a2": "n",
  "b2": "m",
  "y2": "g",

  "auth": "e",
  "poll_rate": 500,
  "url": "https://truffle-plays.fly.dev/"
}
```

Next to d1_left and the other code is the key that will be pressed when the client recieves that input.
For security purposes this only allows letters and numbers to be put in there.
These should be the same keys to press in the game and we do not support multiple keys being pressed for one input.
Next put the password you set when setting up the server where e is and then remove the e.
poll_rate represents how often the client talks to the server we reccomend 500 which means every half second they will talk and get an input.
If you have a large chat make this number smaller.
Lastly set url to the url provided from fly.io on your apps overview page.
You are now set to go with the client that runs on your computer.
Remember if you change a value in this file you must restart the client for it to take effect.

### Viewer client

You are almost done setting this up the last thing is letting your viewers send inputs.
You need to add our embed on your side to truffle.

### Congratulations!

You are now done setting this up and are good to use it.
Have fun.
