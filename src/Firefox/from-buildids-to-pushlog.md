# From build IDs to push log

via [@chutten](https://github.com/chutten):

Found a regression? Here's how to get a pushlog:

1. You have the build dates and you're gonna need revisions. Find the build before the regression and the build after the regression in this list: <https://hg.mozilla.org/mozilla-central/firefoxreleases>
   You want to record the Revision column someplace.
   
       May 10 final f44e64a61ed1
       May 11 final 61a83cc0b74b

2. Put the revisions in this template: `https://hg.mozilla.org/mozilla-central/pushloghtml?fromchange={}&tochange={}`  
   E.g. <https://hg.mozilla.org/mozilla-central/pushloghtml?fromchange=f44e64a61ed1&tochange=61a83cc0b74b>
