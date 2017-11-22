# friend-voter
Friend Voter takes a CSV file with any voter data with the headers FirstName and LastName and compares your Facebook friends.htm file to see if they appear on that list.

This project was created as a part of a GOTV effort for school board elections in 2017.
A campaign manager would take a CSV file with voter data and compare it to your Facebook friends list after the data was exported.
I'd like to improve this project in several ways for the midterm election in 2018 if it's feasible. **Submit an issue** if you have a suggestion or see a problem!

The output contained a yeslist.txt and maybelist.txt. The yeslist contains exact matches. The first and last name match the voter list exactly.
The maybelist contains partial matches. The last name matched. This won't be useful for people with very common last names and will contain duplicates.

# How To Use
These instructions are meant to be easy to follow for less tech literate folks. Please let me know how I can improve if you have any problems.

In order to use this project you'll have to [install Rust](https://www.rust-lang.org/en-US/install.html), [clone the project](https://help.github.com/articles/cloning-a-repository/) to your machine, and either do a ```cargo run [csv file] [htm facebook friends file]``` command
or you'll want to ```cargo build --release``` and grab the executable from the target/release directory and follow the directions below if you are on a Mac (or google the equivalent of these instructions if you're on a different machine).

**Setup Instructions (Mac specific)**
1) Make a new folder called "Friend Voter" on your desktop
2) Put the files you want to use "friend_voter" on inside the folder
3) Open Terminal
4) Type into Terminal ```cd Desktop/Friend\ Voter/``` and hit Enter
5) Type into Terminal ```chmod +x friend_voter``` and hit Enter
6) Type into Terminal to run program ```./friend_voter [csv file] [htm facebook friends file]``` and hit Enter.

An example of this command would be:

```./friend_voter Voted102920171030-6834118298.csv friends.htm```

It must be entered in this order!

7) The results of the program will appear in folder you ran the program.
8) yeslist.txt and maybelist.txt are overwritten everytime you run the program. Rename the results you want to keep or move them out of the folder if you run the program again, but want to keep your results.

**Getting Facebook data**

1) Go to your Facebook Settings.
2) Under the General Account Settings at the bottom of the screen there is a "Download a copy of your Facebook data"
3) Click on the "Download a copy" link and the data will be sent to your email in a couple minutes.
4) Find the friends.htm file and move that into your Friend Voter folder.

**Remove duplicates**

To remove duplicates from maybelist.txt you can run this command in terminal in the folder containing you file:

```sort maybelist.txt | uniq -u > newmaybelist.txt```
