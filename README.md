# chess-ai

## Installation instructions for Ubuntu Linux:
Open terminal by pressing CTRL + ALT + T.
Enter the following commands.

python3 --version
   This command should display a python version. If you do not have python install it from here: https://www.python.org/downloads/
   
cd Documents

git clone https://github.com/piperdaniel1/chess-ai.git

cd chess-ai

sudo apt update  
    You will have to enter your password
    
sudo apt install python3-pip

pip3 install -r requirements.txt  
    If this fails you could try the command python3 -m pip3 install -r requirements.txt

## Running the AI
Now you have installed the AI. To run it you can easily use these commands:  
cd ~/Documents/chess-ai  
    There's no need to run this command if you are already in this directory in the terminal.  
    You will already be in this directory if you just followed the installation instructions.  
      
python3 PyGameWrapper.py  

## Playing the AI
Once you run the game a board will open and you will be playing as White against the AI who is playing Black.
Time starts immediately, you have ten minutes to play your moves with no move bonus. The AI has the same.
It will dynamically adjust how long it takes to make a move based on how much time it has left.
Monitor the terminal during the game. When the game is over, a message will be displayed there.

If the game ever crashes take a picture of the full terminal readout starting from the message that says "Oh no... looks like we hit an unhandled exception."
This will be very helpful for debugging.

You currently cannot change what color the AI plays.
