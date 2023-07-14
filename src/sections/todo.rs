use crate::blocks::paragraph::paragraph;
use crate::containers::Container;
use crate::section_attrs::sec_attrs;
use crate::sections::Section;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum TodoStatus {
    Done,
    NotDone,
    Other(String),
}

pub fn todo(source: &str) -> IResult<&str, Section> {
    let (source, _) = tuple((tag_no_case("-- todo"), not_line_ending, line_ending))(source.trim())?;
    let (source, content) = alt((take_until("\n\n--"), rest))(source.trim())?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (content, paragraphs) = many_till(paragraph, alt((peek(tag("[")), eof)))(content.trim())?;

    let (_content, raw_items) = many0(tuple((
        multispace0,
        tag("["),
        opt(is_not("]")),
        tag("]"),
        multispace1,
        alt((take_until("\n["), rest)).map(|x| many_till(paragraph, eof)(x).unwrap().1 .0),
    )))(content)?;

    let items: Vec<_> = raw_items
        .into_iter()
        .map(|i| match i.2 {
            None => Container::TodoItem {
                status: TodoStatus::NotDone,
                paragraphs: i.5,
            },
            Some(_x) => Container::TodoItem {
                status: TodoStatus::Done,
                paragraphs: i.5,
            },
        })
        .collect();

    Ok((
        source,
        Section::Todo {
            attrs,
            paragraphs: paragraphs.0,
            items,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    // use crate::blocks::Block;
    use crate::sections::Section;
    // use crate::tags::Tag;
    use rstest::rstest;

    #[rstest]
    // This is an effor to try to see what's up with this
    // file that throws an error. It doesn't seem to
    // be at this level
    #[ignore]
    #[case(
r#"
-- title

Stream Ideas

Things to do on stream in case I can't think of anything else.

-- todo

[] Setup to see more than ten items in the grimoire list

[] Make a browser extension that saves bookmark files for you to pull them into the grimoire

[] Make a font command

[] Finish grimoire migration to neopolitan

[] Finish site migration to neopolitan

[] Build chrome extension to pull titles and links for bookmark pages. Maybe make straight from chrome?

[] Make a stand alon grimoire app

[] update ffmpeg gif maker to keep gifs under a certain size

[] Move all the subdomains over to the main site

[] Build a TUI that does... something

[] Update 'hacking.alanwsmith.com' to have music

[] Make a tool that switches OBS to focus on the current active window so you can use more screen realestate

[] Make a bunch of examples for examples.alanwsmith.com with Harvard Sentences and words and alphabets

[] Make Band Names Script for the site

[] Pull data from the Reddit API (two days old - Make a file with the raw contents)

[] Setup links to javascript-playground on main site

[] Make a script that lists apps showing which ones to close

[] Bring back online - Event Bot

[x] Get audio delay fix

[x] Make a !today command

[] Publish to Netlify via local build

[] Setup VNC or whatever to drive the windows machine from the mac

[] Get Start binary clock running

[] Setup Stream Notes again?

[] Build script to close apps that aren't on a clear list

[] Get Live captions working again with better accuracy

[] An app that just makes a water drop sound every 30 minutes reminding you to drink without flashing sometime in front of you visually

[] Build a script to automatically pull new music you like every week to make a post from it - should be able to use this - https://www.youtube.com/playlist?list=LLsEI8203j3l8xWrQKcjs-Jg and filter the type on music, maybe?

[] Setup YouTube embed on the site so you can set a start and end time

[] Figure out how to get rtr.alanwsmith.com to have the next centered with all the lines visible and the max size to fill the page. 

[] write the process to process images for photos.alanwsmith.com as part of the dev process - could throw it to something else, but that makes sense for now since you'll be reviewing the images

[] Show a placeholder image while a new image is coming down form cloudinary-image-maker

[] Add a button to copy the URL to the clipboard

[] Auto-scroll up when you click an example on cloudinary-image-maker

[] Style cloudinary-image-maker

[] Add base samples to cloudinary-image-maker

[] Add syntax highlighting and formatting to json in cloudinary-image-maker

[] Add OG images to alphabet sites and main letters site

[] Upload a custom font to cloudinary for OG image usage

[] Add links to non-posts on the www site

[] Add links to non-www sites in lists of posts on www site

[] make a form that forwards to mdn.io to punt to the mdn docs. 

[] Setup a feature where if you hit the www site from another sub-domain that it has that sub-domain in the nav via a cookie 

[] Make your own examples of the CSS Layout Cookbook from MDN

[] Make an OG image for the www site that works more like dark mode

[] Setup yabai script to only try to move apps if they are already open

[] Figure out why next isn't resizing images for the www site like I thought it was

[] See if there's a way to speed up 'www' site builds with incremental regeneration (or whatever it's called)

[] See if you can make generative art or at least pseudo-random backgrounds for OG images, from Cloudinary

[] Put Scratch and Draft headers on posts based off status to let folks know where things are at

[] Setup a hook to catch a notification when a site build is done (or failed) on Netlify

[] Adjust the content width and H2 sizing on the www site

[] Switch over to using custom component code blocks for syntax highlighting instead of the embedded stuff that messes with the dom

[] Setup to automatically updates pages when you make changes in the grimoire

[] Build a site deployer that doesn't push if there are npm run build errors

[] Setup some ascii art for the terminal for when you start the stream

[] Get TheBotOfAlan back online

[] Move TheBotOfAlan to Windows so it can interact easier with OBS?

[] Setup LayoutMain metadata coming from _app useState to make it easier to setup descriptions, etc... on the www site

[x] Setup an expanso posts text replacement 

[x] Make tl, t2, t3, t4 with tree -L and the number for the number ones and put them in the dot files

[] alphabetize www site posts

[] move www site into 'site' directory

[] toggle to order www site posts by date

[] make syncher timer for time syncing

[] make vanity-tags.alanwsmith.com

[] Make !happy command that sends Snoopy dancing across the screen

[] get !gif back online

[] Make 404 page for next-js starter template not be scalding bright like the sun

[] Make a script that lists all open apps and shows you which ones are clear listed and which should be closed and maybe setup to close them

[] Work on the site (see list)

[] Setup surftech.tv

[] Update monocrack to insert the picker in a way that keeps the lines the same without shifting words off.

[] Create an app that plays a night time playlist for you of youtube music with a sleep.

[] Build a new local launchpad site with next.js and some type of database

[] Get the google chrome extension working for archiving tabs

[] Add !dotfiles command 

[] Make a formatter for podcast transcripts

[] Setup this on the site: https://github.com/agneym/playground

[] Make a site auto deployer

[] Update this with notes on timeout - https://stackoverflow.com/questions/69246187/how-can-i-use-the-leader-key-immediately-after-hitting-escape-in-neovim

[] Make a commands pages on the site that's automatically updated

[] Make dotfiles atomic 

[] Make a Google Chrome Pinboard Extension (assuming it doesn't already exist) 

[] Setup to add new commands to the bot via chat (or whatever)

[] Make some type of space invaders game

[] Finish up your nasa videos with free music thing

[] See about setting up a sqlite3 database to manage some of the content on the site

[] Start making a tools page with forms that submit to use places

[] Check out Microsoft Teams' Voice Recognition CC to compare against web speech api

[] Get the binary clock running again

[] Do a comparison of AWS, Teams (Probably Azure AI), Web Speech, etc... for voice recogintion

[] StackOverflow scraper that looks at python code samples and figure out if they are missing import statements

[] Put pinboard links up on the site somehow

[] Setup ffmpeg to make webp instead of animated gifs

[] Setup so that people can tag quotes somehow through chat and have it automatically pull a video clip of what you said vs what shows up

[] Build a stream-prep script that does everything that can be automated

[] Get event bot working again

[] Add ability to close apps with the yabai resize script

[] Update yabai resize script to be more of template where you do it more declaratively that would also let it be more efficient (maybe)

[] Setup so chat can raise and lower the volume of the music. (maybe just lower...) (Maybe not actually a good idea, but think about it)

[] Get Stream Notes set back up

[] Pull down KEXP Song Of The Day

[] Get binary clock working again (maybe make it a web page?)

[] Make a font with ligatures or whatever so you can stack high and low temperatures in the command line

[] Figure out how to get ASL on the podcasts

[] Extension that shows the current set of links that you're on 

[] Setup bot to show alert when a new chat thing comes in.

[] Figure out why clipping isn't working

[] Make a 3js plot of terminal colors

[] Make a zsh ENV variable to turn off git status in the prompt (possibly other stuff too)

[] Hangman game

[] Build the shout out bot that shouts out after 1 minute if you don't

[] Put search engine on launch page for the site

[] Make a /tools page with handy forms that you can bookmark

[] Setup so that the bot times itself out for a couple of seconds between responses and has a limit of the number of times in the queue

[] Create !audio command with details on gear

[] Moonlander movie

[] Tool to populate grimoire search engine

[] Some script or whatever that opens Day One so it's the first thing I see when I log on in the morning

[] Update plenary test to only show red or green as appropriate instead of both at the same time regardless.

[] Make command line responsive to the number of characters wide the terminal window is

[] Go through all the themes on and print them out with ':runtime syntax/hitest.vim' and then ':TOhtml'

[] Make Emotes

[] Do a test to see how many pinboard links from across the time you've used it are dead

[] Run the process and do stats on the number of links that are dead from the million pixel page

[] Setup bttv and the other thing

[] Soundboard with (and then it exploded sound from galaxy quest)

[] Setup script to capture chrome tabs like you do the safari ones

[] Make Ideas checklist it's own thing

[] Make a file processor that translates stuff for you to remove http and turn links to https for the site

[] Make a checklist of TODOs for the site.

[] Update markdowntable.com so it's live

[] Make a kabob command that takes an string and makes it kabob case and puts it on the pasteboard

[] command to let folks send in new names

[] Make !followage command

[] Setup WebSpeech API to use vocabulary from the grimoire - via Rodriyabala

[] Make one of those floating letter eye control spelling things that hopefully that makes sense 

[] Make a 3D version of the color chart for the base colors in a shell in D3 js or whatever that 3d program is

[] Make a counter that shows the number of tabs you have open on screen

[] Recreate the thing with this video where there's a ton of them, but with like a half second delay

[] Make a stand alone app for YouTube Music

[] Make a local tools website

[] Chrome Extension that lets you hightlight text and then then copies it and the URL to your pasteboard in as a markdown link

[] To Watch viewer that works off pinboard and saves the spot where you are in each video and lets you take notes on the page that then go back into your grimoire

[] Add back in a hot tub switch for channel points

[] Add a followage bot

[] Figure out oauth for eventbot and snapcam bot so they can both work

[] Add sounds to snap cam changes

[] Parse out all the Name of My New Band things you did on twitter (does not look like the search has all of them)

[] Twitter bot to make Name Of My New Bot - 

[] Setup a sound board with 'and it exploded' from galaxy quest - https://www.youtube.com/watch?v=_wMD0ZCh2Sc

[] Get oauth stuff working with event bot and snapcam bot so they work together

[] Make a channel points reward for saying a word or short phrase (that's in english)

[] Make an 'Alan! Alan! Alan!' command from the prairie dog video (also, Steve)

[] Make Please hold music for when you get raided and folks are having to sit through the ads. 

[] Build better playlist(s)

[] Open source snap cam channel points thing

[] Setup nice video for warm-up scene

[] Bound :noh to leader o, (or whatever) to clear searches 

[] Create states on the number of times each snapcam filter is used

[] Make a &quot;commands&quot; command that lists all your commands

[] Make an &quot;!saythis&quot; command that lets you add new command responses

[] Place your bets hotkey for 'does it work or not'

[] Finish up your nasa videos with free music thing

[] Check out Microsoft Teams' Voice Recognition CC to compare against web speech api

[] Do a comparison of AWS, Teams (Probably Azure AI), Web Speech, etc... for voice recogintion

[] Setup so that people can tag quotes somehow through chat and have it automatically pull a video clip of what you said vs what shows up

[] Extension that shows the number of tabs you have open in chrome

[] Make a font with ligatures or whatever so you can stack high and low temperatures in the command line

[] Extension that shows the current set of links that you're on 

[] Setup bot to show alert when a new chat thing comes in. 

[] Figure out why clipping isn't working

[] Make a 3js plot of terminal colors

[] Make a zsh ENV variable to turn off git status in the prompt (possibly other stuff too)

[] Hangman game

[] Build the shout out bot that shouts out after 1 minute if you don't

[] Put search engine on launch page for the site

[] Setup so that the bot times itself out for a couple of seconds between responses and has a limit of the number of times in the queue

[] Create !audio command with details on gear

[] Moonlander movie

[] Tool to populate grimoire search engine

[] Some script or whatever that opens Day One so it's the first thing I see when I log on in the morning

[] Update plenary test to only show red or green as appropriate instead of both at the same time regardless. 

[] Make command line responsive to the number of characters wide the terminal window is

[] Go through all the themes on and print them out with ':runtime syntax/hitest.vim' and then ':TOhtml'

[] Make Emotes

[] Setup bttv and the other thing 

[] Setup so folks can send in foxdot to play their own intros (limited to n-number of seconds)

[] Make a '!meow' command with: 'https://clips.twitch.tv/HeadstrongDepressedFalconDoggo-sozru2ujyLQ2mPoi'

[] Create a command that sorts tabs chronologically (and by domain, or maybe alphabetically)

[] Create a 30 second raid countdown timer so you know when folks are past the ads

[] Create a rust app that replaces the pyenv stuff to switch to virtualenv environments when you move into the dir so you don't have to have a slow prompt with the default way. via nyxiative. 

[] Make a thing out of these where they randomly update on a grid: https://en.wikipedia.org/wiki/Box-drawing_character

[] Make a color sorter game that's just kinda a sudoku type thing based of what you were doing for the terminal colors. Setup with differnt hues and let folks sort them. Just a fidget game

[] Make a rust command line app called 'kebab-case' that turns things into kebab case. Same goes for 'snake-case'. Mayebe make the app called 'caseit' then pass the type you want so you can add different types. Maybe have a default of kebab. Or store a default

[] Make a mac app (or electron app) that shows you raids and such and lets you tick things off. Idea is to have something that pops up on your mac so you don't miss the notification on your stream pc

[] Update the markdowntable formatter. 

[] Add 'followage' command that shows how long someone has followed

[] Create a '!x####' command that shows an XKCD

[] Setup an interface to pinboard Read Later files for a reader - Show partial reads. Maybe make it it's own app for the iPad? and tie it into your news letters?

[] Create a rust app that does the switch to python virtual envioronments like the 'pyenv' but faster

[] Create a reader type app that you can link up with pinboard to create a reading list and video watch list and use it for things like reading the python or neovim docs (gotta figure out how to feed it and bookmark it to show progress)

[] Get stuff from here to make scenes for BRB and let folks change them with chat: https://archive.org/details/anime

[] Update the Meilisearch page to point out that IDs have to be integers

[] Make a !captions command that turns on live captions

[] !uptime command via WaylonWalker

[] Make a browser source for chat that just puts messages in random locations. because why not

[] Setup a website that collects examples for different documentation things with links to them. Or, a version of the does that has them, like your own fork, that you can submit back, but have available if they aren't accepted. 

[] Update this page to show how to use a 'lua << EOF' here doc to do the require for the config - https://github.com/nvim-telescope/telescope-fzf-native.nvim

[] Get .dotfiles files published and linked and setup in a '!dotfiles' command

[] Make an 'I'm eating so the facecam is off' scene

[] Setup a vocal queue for '!lurk' that uses the recordings of folks names to give them a lurk response. Could use that as a way to point folks to the name pronunciation website. 

[] Make sure the bot has a govnah so it doesn't get timed out.

[] Make a '!done' command that done 'And that's what I think about that' from forest gump - with a graphic of a tripple front flip holding a  in a chair. or something, like maybe a chainsaw? (the flip idea via yetanothernull)

[] Ongoing project: Go through python (and other language) docs and make examples (e.g. https://docs.python.org/3/library/random.html#random.randint)

[] Auto publish gimoire  

[] Make a graph of the frequency of changes to the moonlander

[] Make a !hype command (TBD on what do to with it)

[] Add !picklealan (via Bobross)

[] Setup stream parrot - https://streamparrot.com

[] Setup TwitchIO 2 (when webhooks and pubsub are in)

[] Make a ~corgi command that does something cool

[] Make a credits page with lists of the ideas: - Memory Leek - SheWhoExists - rexroof see below - owenkbcodes see below

[] pulse background when alerts happen via rexroof

[] Make an alan grant taking off sunglasses thing for followers via: owenkbcodes

[] Make a !stuckbox command 

[] Marque with emotes where folks can do '!marque EMOTE' and have it scorll across (do like ten of them in whatever pattern folks put in)

[] Add '!song', '!np', and '!music' commands that get the song tha'ts currenly playing on MonsterCat if they have an API. adn then '!sr' for song request

[] Do an HTML version of this: https://www.reddit.com/r/webdev/comments/nm6wcl/18_cards_of_how_to_design_web_forms/

[] App that's just a chromium wrapper for watching twitch so you can keep it as separate app

[] Walk thru video of getting various twitch credential.

[] auto update profile image avatar

[] !gi also work for ~gif

[] Make '!attack' and '!defend' commands for raids

[] Fire '!gif' periodically

[] galaxy quest laugh

[] snake game in the browser

[] consider a zoom feature that blows up the editor window bigger

[] !chipmonk command to pitch shift audio

[] Make a '!clip' command to make clip

[] Add an '!addquote' command that collects quotes and does something with them

[] Automatic ASL captions

[] Add aria labels to your site

[] Sound board with chat triggers - Well Played, Golf Clap

[] Consollidate chatbot, eventsub, and obs web sockets

[] setup stream trailer

[] make emotes - eyebrow raised, two thumbs up, huzzah

[] Fix gif cropping

[] Make !picklealan

[] Make a '!help' command

[] Setup Twitch PubSub for TheBotOfAlan

[] Command to setup a new project that makes a repo, clones it, drops in .gitignore, and touches a README file

[] Figure out how to send audio queues to OBS

[] Mess around with TryHackMe

[] Setup url capture tool to automatically start/stop with stream for stream notes. 

[] Setup: https://mycroft.ai/

[] add !clap command with clapping gif. possibly random

[] Setup weather display for launchpad

[] Setup a deployer for tools

[] Setup your own stream chat view with photos for the people

[] Auto shout out raiders

[] Gif game. clip to a part of a gif randomly and let folks guess what it's from (multiple choice) - Figure out how to keep points. Expand the gif as people make guesses which reduces the number of points you can get. 

[] Setup a process to show a 1 image per second 'video' with audio synced as an alt to twitch bandwidth problems for folks like null. 

[] Setup process to show whos streaming on launchpad so you can see who's live and click to them directly

[] Setup so that folks have their own walk-up music when they first say hi in chat. Maybe have a server that lets them listen to music queues and pick on and then have that play. 

[] Do something with Superbase

[] let folks control your terminal colors

[] show the last emote that was used in chat

[] teleport command that moves where you are in the scene

[] Go through OBS WebSocket-py and get settings for all the various things to make a blog post out of. 

[] Parse Discord history to look at your reactions and count how many times you use the various emotes

[] Setup a bot that watches wordstonks and makes a guess every 20 seconds so folks have to race against the bot. Should slow down toward the end, otherwise it'll win most of the time. Gotta fine tune the time, really

[] Show how to get the different types of access tokesn from twitch (User Access, ID, and App Access) on this page: https://dev.twitch.tv/docs/authentication#getting-tokens

[] Command to update a quote in the scene

[] Command to update the background

[] Command to post a random idea

[] Command to pick a random word to see if you can come up with an idea for it

[] Look at what's in Twitch PubSub to see what you can turn on with that and maybe use that instead of EventSub since it looks like you wouldn't have to setup ngrok (which you should probably setup anyway)

[] Setup the Ghost in the Shell drop animation

[] Will it work or not poll. (even though twitch has their own polls)

[] Build a script the just posts a random harvard sentence to a twitter account or to OBS

[] Play: https://screeps.com

[] Play: https://danielyxie.github.io/bitburner/

[] Play: https://www.zachtronics.com/exapunks/

[] Show what song you're listening to in spotify

[] Get Twitch Bot back up and running

[] Make an !xkcd command that shows an image on the screen

[] Make an animation of your keyboard changes, maybe with that react video maker or ffmpeg

[] Move stream notes to go to gatsby content area

[] Finish the red/green highlight updater for python unittest. 

[] Make a quick diff tool that you can paste two things in and maybe pretty print and then see the diffs. 

[] Make a MOTD (message of the day) to show your commands so you can learn them better

[] Build an Anki style learning system into your home page. 

[] Setup command that lets chat change the filterset on the facecam

[] Build a tool where chat can say their names into a browser so you can hear how to pronounce them

[] Make a search engine for the gimoire because.... it's not really a good idea, but you'd learn a lot

[] Create a script that shows random UTF-8 characters. maybe some fade in/out stuff. 

[] Make a bunch of sample/example sets of data

[] Mess with EMR Notebooks and Studio

[] Rate Programming language logos

[] Setup a process that screenshots the screen once every second and sends it to a web server for viewing by folks who's internet isn't workign well. 

[] Setup an app to track your ideas for the stream

[] Setup an app to track whos who with different networks (basically a cross map)

[] Make things like his for your launch page: https://percentagecalculator.net/

[] Mess around with this: https://beta.decapi.me/twitch/uptime/theidofalan

[] Cut out a Prince's Hot Takes for him here: https://clips.twitch.tv/CoyApatheticYakinikuDAESuppy-d-83uGGYUwg37b7P

[] Make ASCII art for terminal to start up stream

[] Setup this on the site: https://github.com/agneym/playground

[] Make a site auto deployer

[] Make a formatter for podcast transcripts

[] Add top level '<Checklist />' component to site

[] Update this with notes on timeout - https://stackoverflow.com/questions/69246187/how-can-i-use-the-leader-key-immediately-after-hitting-escape-in-neovim

[] Make a commands pages on the site that's automatically updated

[] Make dotfiles atomic 

[] Make a Google Chrome Pinboard Extension (assuming it doesn't already exist) 

[] Setup so we can add new commands to the bot via chat (or whatever)

[] Make some type of space invaders game

[] Finish up your nasa videos with free music thing

[] Add !dotfiles command 

[] See about setting up a sqlite3 database to manage some of the content on the site

[] Start making a tools page with forms that submit to use places

[] Check out Microsoft Teams' Voice Recognition CC to compare against web speech api

[] Get the binary clock running again

[] Do a comparison of AWS, Teams (Probably Azure AI), Web Speech, etc... for voice recogintion

[] StackOverflow scraper that looks at python code samples and figure out if they are missing import statements

[] Setup so that people can tag quotes somehow through chat and have it automatically pull a video clip of what you said vs what shows up

[] Build a stream-prep script that does everything that can be automated

[] Get event bot working again

[] Make a font with ligatures or whatever so you can stack high and low temperatures in the command line

[] Figure out how to get ASL on the podcasts

[] Extension that shows the current set of links that you're on 

[] Setup bot to show alert when a new chat thing comes in.

[] Figure out why clipping isn't working

[] Make a 3js plot of terminal colors

[] Make a zsh ENV variable to turn off git status in the prompt (possibly other stuff too)

[] Hangman game

[] Build the shout out bot that shouts out after 1 minute if you don't

[] Put search engine on launch page for the site

[] Make a /tools page with handy forms that you can bookmark

[] Setup so that the bot times itself out for a couple of seconds between responses and has a limit of the number of times in the queue

[] Create !audio command with details on gear

[] Moonlander movie

[] Tool to populate grimoire search engine

[] Some script or whatever that opens Day One so it's the first thing I see when I log on in the morning

[] Update plenary test to only show red or green as appropriate instead of both at the same time regardless.

[] Make command line responsive to the number of characters wide the terminal window is

[] Go through all the themes on and print them out with ':runtime syntax/hitest.vim' and then ':TOhtml'

[] Make Emotes

[] Setup bttv and the other thing

[] Soundboard with (and then it exploded sound from galaxy quest)

[] Setup script to capture chrome tabs like you do the safari ones

[] Make Ideas checklist it's own thing

[] Make a file processor that translates stuff for you to remove http and turn links to https for the site

[] Make a checklist of TODOs for the site.

[] Update markdowntable.com so it's live

[] Make a kabob command that takes an string and makes it kabob case and puts it on the pasteboard

[] command to let folks send in new names

[] Make !followage command

[] Setup WebSpeech API to use vocabulary from the grimoire - via Rodriyabala

[] Make one of those floating letter eye control spelling things that hopefully that makes sense 

[] Make a 3D version of the color chart for the base colors in a shell in D3 js or whatever that 3d program is

[] Make a counter that shows the number of tabs you have open on screen

[] Recreate the thing with this video where there's a ton of them, but with like a half second delay

[] Make a stand alone app for YouTube Music

[] Make a local tools website

[] Chrome Extension that lets you hightlight text and then then copies it and the URL to your pasteboard in as a markdown link

[] To Watch viewer that works off pinboard and saves the spot where you are in each video and lets you take notes on the page that then go back into your grimoire

[] Add back in a hot tub switch for channel points

[] Add a followage bot

[] Figure out oauth for eventbot and snapcam bot so they can both work

[] Add sounds to snap cam changes

[] Parse out all the Name of My New Band things you did on twitter (does not look like the search has all of them)

[] Twitter bot to make Name Of My New Bot - 

[] Setup a sound board with 'and it exploded' from galaxy quest - https://www.youtube.com/watch?v=_wMD0ZCh2Sc

[] Get oauth stuff working with event bot and snapcam bot so they work together

[] Make a channel points reward for saying a word or short phrase (that's in english)

[] Make an 'Alan! Alan! Alan!' command from the prairie dog video (also, Steve)

[] Make Please hold music for when you get raided and folks are having to sit through the ads. 

[] Build better playlist(s)

[] Open source snap cam channel points thing

[] Setup nice video for warm-up scene

[] Bound :noh to leader o, (or whatever) to clear searches 

[] Create states on the number of times each snapcam filter is used

[] Make a &quot;commands&quot; command that lists all your commands

[] Make an &quot;!saythis&quot; command that lets you add new command responses

[] Place your bets hotkey for 'does it work or not'

[] Finish up your nasa videos with free music thing

[] Check out Microsoft Teams' Voice Recognition CC to compare against web speech api

[] Do a comparison of AWS, Teams (Probably Azure AI), Web Speech, etc... for voice recogintion

[] Setup so that people can tag quotes somehow through chat and have it automatically pull a video clip of what you said vs what shows up

[] Extension that shows the number of tabs you have open in chrome

[] Make a font with ligatures or whatever so you can stack high and low temperatures in the command line

[] Extension that shows the current set of links that you're on 

[] Setup bot to show alert when a new chat thing comes in. 

[] Figure out why clipping isn't working

[] Make a 3js plot of terminal colors

[] Make a zsh ENV variable to turn off git status in the prompt (possibly other stuff too)

[] Hangman game

[] Build the shout out bot that shouts out after 1 minute if you don't

[] Put search engine on launch page for the site

[] Setup so that the bot times itself out for a couple of seconds between responses and has a limit of the number of times in the queue

[] Create !audio command with details on gear

[] Moonlander movie

[] Tool to populate grimoire search engine

[] Some script or whatever that opens Day One so it's the first thing I see when I log on in the morning

[] Update plenary test to only show red or green as appropriate instead of both at the same time regardless. 

[] Make command line responsive to the number of characters wide the terminal window is

[] Go through all the themes on and print them out with ':runtime syntax/hitest.vim' and then ':TOhtml'

[] Make Emotes

[] Setup bttv and the other thing 

[] Setup so folks can send in foxdot to play their own intros (limited to n-number of seconds)

[] Make a '!meow' command with: 'https://clips.twitch.tv/HeadstrongDepressedFalconDoggo-sozru2ujyLQ2mPoi'

[] Create a command that sorts tabs chronologically (and by domain, or maybe alphabetically)

[] Create a 30 second raid countdown timer so you know when folks are past the ads

[] Create a rust app that replaces the pyenv stuff to switch to virtualenv environments when you move into the dir so you don't have to have a slow prompt with the default way. via nyxiative. 

[] Make a thing out of these where they randomly update on a grid: https://en.wikipedia.org/wiki/Box-drawing_character

[] Make a color sorter game that's just kinda a sudoku type thing based of what you were doing for the terminal colors. Setup with differnt hues and let folks sort them. Just a fidget game

[] Make a rust command line app called 'kebab-case' that turns things into kebab case. Same goes for 'snake-case'. Mayebe make the app called 'caseit' then pass the type you want so you can add different types. Maybe have a default of kebab. Or store a default

[] Make a mac app (or electron app) that shows you raids and such and lets you tick things off. Idea is to have something that pops up on your mac so you don't miss the notification on your stream pc

[] Update the markdowntable formatter. 

[] Add 'followage' command that shows how long someone has followed

[] Create a '!x####' command that shows an XKCD

[] Setup an interface to pinboard Read Later files for a reader - Show partial reads. Maybe make it it's own app for the iPad? and tie it into your news letters?

[] Create a rust app that does the switch to python virtual envioronments like the 'pyenv' but faster

[] Create a reader type app that you can link up with pinboard to create a reading list and video watch list and use it for things like reading the python or neovim docs (gotta figure out how to feed it and bookmark it to show progress)

[] Get stuff from here to make scenes for BRB and let folks change them with chat: https://archive.org/details/anime

[] Update the Meilisearch page to point out that IDs have to be integers

[] Make a !captions command that turns on live captions

[] !uptime command via WaylonWalker

[] Make a browser source for chat that just puts messages in random locations. because why not

[] Setup a website that collects examples for different documentation things with links to them. Or, a version of the does that has them, like your own fork, that you can submit back, but have available if they aren't accepted. 

[] Update this page to show how to use a 'lua << EOF' here doc to do the require for the config - https://github.com/nvim-telescope/telescope-fzf-native.nvim

[] Get .dotfiles files published and linked and setup in a '!dotfiles' command

[] Make an 'I'm eating so the facecam is off' scene

[] Setup a vocal queue for '!lurk' that uses the recordings of folks names to give them a lurk response. Could use that as a way to point folks to the name pronunciation website. 

[] Make sure the bot has a govnah so it doesn't get timed out.

[] Make a '!done' command that done 'And that's what I think about that' from forest gump - with a graphic of a tripple front flip holding a  in a chair. or something, like maybe a chainsaw? (the flip idea via yetanothernull)

[] Ongoing project: Go through python (and other language) docs and make examples (e.g. https://docs.python.org/3/library/random.html#random.randint)

[] Auto publish gimoire  

[] Make a graph of the frequency of changes to the moonlander

[] Make a !hype command (TBD on what do to with it)

[] Setup stream parrot - https://streamparrot.com

[] Setup TwitchIO 2 (when webhooks and pubsub are in)

[] Make a ~corgi command that does something cool

[] Make a credits page with lists of the ideas: - Memory Leek - SheWhoExists - rexroof see below - owenkbcodes see below

[] pulse background when alerts happen via rexroof

[] Make an alan grant taking off sunglasses thing for followers via: owenkbcodes

[] Make a !stuckbox command 

[] Marque with emotes where folks can do '!marque EMOTE' and have it scorll across (do like ten of them in whatever pattern folks put in)

[] Add '!song', '!np', and '!music' commands that get the song tha'ts currenly playing on MonsterCat if they have an API. adn then '!sr' for song request

[] Do an HTML version of this: https://www.reddit.com/r/webdev/comments/nm6wcl/18_cards_of_how_to_design_web_forms/

[] App that's just a chromium wrapper for watching twitch so you can keep it as separate app

[] Walk thru video of getting various twitch credential.

[] auto update profile image avatar

[] !gi also work for ~gif

[] Make '!attack' and '!defend' commands for raids

[] Fire '!gif' periodically

[] galaxy quest laugh

[] snake game in the browser

[] consider a zoom feature that blows up the editor window bigger

[] !chipmonk command to pitch shift audio

[] Make a '!clip' command to make clip

[] Add an '!addquote' command that collects quotes and does something with them

[] Automatic ASL captions

[] Add aria labels to your site

[] Sound board with chat triggers - Well Played, Golf Clap

[] Consollidate chatbot, eventsub, and obs web sockets

[] setup stream trailer

[] make emotes - eyebrow raised, two thumbs up, huzzah

[] Fix gif cropping

[] Make !picklealan

[] Make a '!help' command

[] Setup Twitch PubSub for TheBotOfAlan

[] Command to setup a new project that makes a repo, clones it, drops in .gitignore, and touches a README file

[] Figure out how to send audio queues to OBS

[] Mess around with TryHackMe

[] Setup url capture tool to automatially start/stop with stream for stream notes. 

[] Setup: https://mycroft.ai/

[] add !clap command with clapping gif. possibly random

[] Setup weather display for launchpad

[] Setup a deployer for tools

[] Setup your own stream chat view with photos for the people

[] Auto shout out raiders

[] Gif game. clip to a part of a gif randomly and let folks guess what it's from (multiple choice) - Figure out how to keep points. Expand the gif as people make guesses which reduces the number of points you can get. 

[] Setup a process to show a 1 image per second 'video' with audio synced as an alt to twitch bandwidth problems for folks like null. 

[] Setup process to show whos streaming on launchpad so you can see who's live and click to them directly

[] Setup so that folks have their own walk-up music when they first say hi in chat. Maybe have a server that lets them listen to music queues and pick on and then have that play. 

[] Do something with Superbase

[] let folks control your terminal colors

[] show the last emote that was used in chat

[] teleport command that moves where you are in the scene

[] Go through OBS WebSocket-py and get settings for all the various things to make a blog post out of. 

[] Parse Discord history to look at your reactions and count how many times you use the various emotes

[] Setup a bot that watches wordstonks and makes a guess every 20 seconds so folks have to race against the bot. Should slow down toward the end, otherwise it'll win most of the time. Gotta fine tune the time, really

[] Show how to get the different types of access tokesn from twitch (User Access, ID, and App Access) on this page: https://dev.twitch.tv/docs/authentication#getting-tokens

[] Command to update a quote in the scene

[] Command to update the background

[] Command to post a random idea

[] Command to pick a random word to see if you can come up with an idea for it

[] Look at what's in Twitch PubSub to see what you can turn on with that and maybe use that instead of EventSub since it looks like you wouldn't have to setup ngrok (which you should probably setup anyway)

[] Setup the Ghost in the Shell drop animation

[] Will it work or not poll. (even though twitch has their own polls)

[] Build a script the just posts a random harvard sentence to a twitter account or to OBS

[] Play: https://screeps.com

[] Play: https://danielyxie.github.io/bitburner/

[] Play: https://www.zachtronics.com/exapunks/

[] Show what song you're listenting to in sportify

[] Get Twitch Bot back up and running

[] Make an !xkcd command that shows an image on the screen

[] Make an animation of your keyboard changes, maybe with that react video maker or ffmpeg

[] Move stream notes to go to gatsby content area

[] Finish the red/green highlight updater for python unittest. 

[] Make a quick diff tool that you can paste two things in and maybe pretty print and then see the diffs. 

[] Make a MOTD (message of the day) to show your commands so you can learn them better

[] Build an Anki style learning system into your home page. 

[] Setup command that lets chat change the filterset on the facecam

[] Build a tool where chat can say their names into a browser so you can hear how to pronounce them

[] Make a search engine for the gimoire because.... it's not really a good idea, but you'd learn a lot

[] Create a script that shows random UTF-8 characters. maybe some fade in/out stuff. 

[] Make a bunch of sample/example sets of data

[] Mess with EMR Notebooks and Studio

[] Rate Programming language logos

[] Setup a process that screenshots the screen once every second and sends it to a web server for viewing by folks who's internet isn't workign well. 

[] Setup an app to track your ideas for the stream

[] Setup an app to track whos who with different networks (basically a cross map)

[] Make things like his for your launch page: https://percentagecalculator.net/

[] Mess around with this: https://beta.decapi.me/twitch/uptime/theidofalan

[] Cut out a Prince's Hot Takes for him here: https://clips.twitch.tv/CoyApatheticYakinikuDAESuppy-d-83uGGYUwg37b7P


-- h2

Completed 


-- todo


[x] Make an easier to use checklist feature for the grimoire

[x] Add top level <ReadOnlyChecklist /> component to site

[x] Migrate site from Gatsby to Nextjs

[x] Make snapcam 'api' for channel points

[x] Add '!fear' command with Fear is the mind killer quote

[x] Setup a git changed indicator for you prompt

[x] Make a single script that fires up both flask and web sockets in the scratchpad

[x] Make: !rtr, !rt, !rolltide

[x] Add a response to '!lurk'

[x] Make a '!blog' command

[x] Make a '!site' command

[x] Make a '!twitter' command

[x] Make a '!skull' command

[x] Make drop graphic command and set it up with the gif

[x] Add OBS Web Socket to TheBotOfAlan

[x] Create a '!so' shoutout command

[x] Add top level <Checklist /> component to site

[x] Add !vim command that points to: https://www.youtube.com/watch?v=H3o4l4GVLW0

[x] Setup a script to make different tinted versions of the podcast show art

[x] Setup a script to migrate the podcast rss feed over to the site

[x] Parse the Egghead IO page to sort courses and create a list of the ones to do

[x] Command to change snap cam filters

[x] Add !vim command that points to: https://www.youtube.com/watch?v=H3o4l4GVLW0

[x] Parse the Egghead IO page to sort courses and create a list of the ones to do

[x] Add '!fear' command with Fear is the mind killer quote

[x] Setup a git changed indicator for you prompt

[x] Make a single script that fires up both flask and web sockets in the scratchpad

[x] Make: !rtr, !rt, !rolltide

[x] Add a response to '!lurk'

[x] Make a '!blog' command

[x] Make a '!site' command

[x] Make a '!twitter' command

[x] Make a '!skull' command

[x] Make drop graphic command and set it up with the gif

[x] Add OBS Web Socket to TheBotOfAlan

[x] Create a '!so' shoutout command

[x] Make snapcam 'api' for channel points

[x] Make sure you can't update the url by hand in the cloudinary image maker

[x] Debounce cloudinary-image-maker so you can auto update the image



-- categories
-- Streaming

-- attributes
-- date: 2021-02-25 13:23:30
-- id: 20eo1slr
-- type: post
-- status: published
-- site: aws

"#,
        Ok(("", Section::None))
)]
    fn solo_test_todo(#[case] input: String, #[case] expected: IResult<&str, Section>) {
        assert_eq!(expected, todo(input.as_str()));
    }

    // #[case(
    //     ["-- todo",
    //         "",
    //         "this is some text",
    //         "",
    //         "[] alfa1",
    //         "alfa2",
    //         "",
    //         "alfa3",
    //         "alfa4",
    //         "",
    //         "[x] bravo1",
    //         "",
    //         "bravo2",
    //         "",
    //         "bravo3",
    //         "",
    //         "-- placeholder"].join("\n"),
    //     Ok(("\n\n-- placeholder",
    //     Section::Todo {
    //         attrs: vec![],
    //         paragraphs: vec![
    //                         Block::Paragraph {
    //                             tags: vec![
    // Tag::Text { text: "this is some text".to_string() }
    //                             ]
    //                         },
    //             ],
    //         items: vec![
    //                 Container::TodoItem {
    //                     status: TodoStatus::NotDone,
    //                     paragraphs:
    //                     vec![
    //                         Block::Paragraph {
    //                             tags: vec![
    // Tag::Text { text: "alfa1 alfa2".to_string() }
    //                             ]
    //                         },
    //                         Block::Paragraph {
    //                             tags: vec![
    // Tag::Text { text: "alfa3 alfa4".to_string() }
    //                             ]
    //                         },
    //                     ]
    //                 },
    //                 Container::TodoItem{
    //                     status: TodoStatus::Done,
    //         paragraphs:
    //                     vec![
    //                         Block::Paragraph {
    //                             tags: vec![
    // Tag::Text { text: "bravo1".to_string() }
    //                             ]
    //                         },
    //                         Block::Paragraph {
    //                             tags: vec![
    // Tag::Text { text: "bravo2".to_string() }
    //                             ]
    //                         },
    //                         Block::Paragraph {
    //                             tags: vec![
    // Tag::Text { text: "bravo3".to_string() }
    //                             ]
    //                         },
    //                     ]
    //                 },
    //             ]
    //     }))
    // )]
    // /

    // fn solo_test_todo(#[case] i: String, #[case] e: IResult<&str, Section>) {
    //     assert_eq!(e, todo(i.as_str()));
    // }

    //
}
