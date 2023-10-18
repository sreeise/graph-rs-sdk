# Contributing

This project welcomes all who would like to contribute. There are multiple ways to get involved. For the most part the current contributors are also users of the crate, so most contributions are code related. But there are plenty of tasks that can be taken up for anyone wanting to contribute and help regardless. 

- [New Contributors](#new-contributors)
- [Contributing to code](#contributing-to-code)
- [If you have industry experience](#related-expertise-and-experience)

# New Contributors 

Because this crate is maintained by only a few people there will not generally be specific bug/tickets that have a first-time contributor label. However, there are a few things first time contributors can do that are easier to start with: 

- Contribute documentation and suggest ways to further enhance documentation. 
- Ask and answer discussions in the Discussions tab. 

For those new to this project and have a technical background whether you use the crate or not: 

- Contribute to the wiki. The wiki is the place where the technical aspects of the crate and its inner workings are documented. 
- Write Rust code examples using this crate. 
- Write detailed issues and provide knowledge on how to implement features or fix bugs. 

# Contributing to Code: 

1. Please contribute to code by first choosing a ticket/issue to work on. If you think there is a issue that needs to be worked on or addressed then feel free to create the issue. It will be reviewed by a maintainer and verified. You can then work on this issue as you please. Let the crate maintainers know you would like to work on the issue also. Feel free to also ask questions and get feedback early and often.
The workflow for contirbutions to code as the same for most projects on GitHub. Fork the project and open a pull request. 

2. When you first open a pull request the tests will probably fail due to non-maintainers not having access to the application secrets stored in GitHub when the action is run on the tests. GitHub blocks actions from accessing these for forks and the GitHub action is running on pull requests. Unfortunately this also means its difficult to verify your change without manual intervention. A maintainer will pull down your change and run the tests locally to verify. Note that when you are running these tests locally you are really only running 1/4 of the total tests. You need the Active Directory credentials to run the others. These cannot just be given out as you can probably imagine and this is also something Microsoft has provided as a service free of charge (developer sandbox) so we have to be respectful to Microsoft and careful with who gets access to this.

3. While this library does not follow completely and has not always followed Microsoft's guidelines for authentication and sdk libraries this goal is to follow these guidelines and adhere to any requirements. Especially security requirements. With that said it is often difficult to find documentation on what those stated guidelines are. If you have knowledge of this, then please reach out.  

4. The crate is mainly divided up into a few categories which are also crates: 

- `graph-http`: Implementation of and execution of requests and http related extensions or abstractions such as paging. 
- `graph-oauth`: OAuth implementation. This should always follow the OAuth 2.0 reference and the Microsoft guidelines where possible. Security issues should be reported using private vulnerability reporting. 
- `src (main crate)`: Houses all api client methods generated and the client itself. 
- `Codegen`: Parsing OpenApi configs and the macro writing of api clients 



# Related expertise and experience

For those with technical expertise in areas such as OAuth and Microsoft Identity Platform, Microsoft libraries including MSAL/Azure/MsGraph, Windows integration, deep knowledge of Rust and experience, and any related areas please reach out to the maintainer through a ticket or discussions tab if you would like to talk with or provide knowledge on these topics. Documentation and/or guidelines are difficult to come by so any industry knowledge is welcome.
