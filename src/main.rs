#![windows_subsystem = "windows"] //Windows GUI application - suppress CLI window

// Import necessary modules and components from the `iced` crate for building the GUI
use iced::{
    alignment::Alignment,              // For aligning widgets within containers
    theme::Theme,                      // Theme for styling the application
    widget::{Button, Column, Container, Text, TextInput}, // UI widgets
    Application,                       // Trait for building the main application
    Command,                           // For handling side effects
    Element,                           // Represents UI elements
    Length,                            // For sizing widgets
    Settings,                          // Application settings
};

// Import the `rand` crate for generating random numbers
use rand::Rng;
// Import the `Ordering` enum for comparing numbers
use std::cmp::Ordering;

// Entry point of the application
pub fn main() -> iced::Result {
    // Run the `GuessingGame` application with default settings
    GuessingGame::run(Settings::default())
}

// Define the main application structure
struct GuessingGame {
    secret_number: u32, // The randomly generated number the user needs to guess
    guess: String,      // The current guess input by the user
    message: String,    // Feedback message displayed to the user
}

// Define the different messages/events that can occur in the application
#[derive(Debug, Clone)]
pub enum Message {
    GuessInputChanged(String), // Triggered when the user changes the input in the text field
    GuessButtonPressed,        // Triggered when the user presses the "Guess" button
}

// Implement the `Application` trait for `GuessingGame`
impl Application for GuessingGame {
    // Specify the executor type for handling asynchronous tasks
    type Executor = iced::executor::Default;
    // Define the type of messages the application will handle
    type Message = Message;
    // Define the theme for the application
    type Theme = Theme; // Ensure Theme is imported correctly
    // Define any flags that might be passed when initializing the application
    type Flags = ();

    // Method to initialize the application
    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // Generate a random secret number between 1 and 100
        let secret_number = rand::thread_rng().gen_range(1..=100);
        (
            GuessingGame {
                secret_number,
                guess: String::new(),                        // Initialize guess as an empty string
                message: String::from("Welcome to the Guessing Game!"), // Initial welcome message
            },
            Command::none(), // No initial commands to run
        )
    }

    // Method to set the window title
    fn title(&self) -> String {
        String::from("Guessing Game") // Title displayed on the application window
    }

    // Method to handle updates based on incoming messages/events
    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            // Handle changes in the guess input field
            Message::GuessInputChanged(value) => {
                self.guess = value; // Update the current guess with the new input
            }
            // Handle the event when the "Guess" button is pressed
            Message::GuessButtonPressed => {
                // Attempt to parse the guess input into an unsigned integer
                let guess: u32 = match self.guess.trim().parse() {
                    Ok(num) => num, // Successfully parsed number
                    Err(_) => {
                        // If parsing fails, update the message to prompt for a valid number
                        self.message = String::from("Please enter a valid number.");
                        return Command::none(); // Exit the update without further processing
                    }
                };

                // Compare the user's guess with the secret number
                match guess.cmp(&self.secret_number) {
                    Ordering::Less => self.message = String::from("Too small!"), // Guess is lower than secret
                    Ordering::Greater => self.message = String::from("Too big!"), // Guess is higher than secret
                    Ordering::Equal => {
                        // Correct guess; inform the user of their success
                        self.message = String::from("You win! 🎉");
                        // Optionally, you could reset the game here by generating a new secret number
                    }
                }

                // Clear the input field after processing the guess
                self.guess.clear();
            }
        }
        Command::none() // No additional commands to run after handling the message
    }

    // Method to define the layout and appearance of the application's UI
    fn view(&self) -> Element<Message> {
        // Create a text input field for the user's guess
        let guess_input = TextInput::new("Enter your guess...", &self.guess) // Placeholder and current value
            .on_input(Message::GuessInputChanged) // Define the message to send on input change
            .padding(10)                          // Add padding inside the text field
            .size(20);                            // Set the font size

        // Create a button that the user can press to submit their guess
        let guess_button = Button::new(Text::new("Guess")) // Button with the label "Guess"
            .on_press(Message::GuessButtonPressed);       // Define the message to send on button press

        // Arrange the UI elements vertically in a column
        let content = Column::new()
            .push(Text::new(&self.message).size(30)) // Display the current message with larger text
            .push(guess_input)                        // Add the guess input field
            .push(guess_button)                       // Add the guess button
            .padding(20)                              // Add padding around the column
            .align_items(Alignment::Center);          // Center-align all items within the column

        // Embed the column inside a container that fills the available space
        Container::new(content)
            .width(Length::Fill)   // Make the container take the full available width
            .height(Length::Fill)  // Make the container take the full available height
            .center_x()             // Center content horizontally
            .center_y()             // Center content vertically
            .into()                 // Convert the container into an `Element<Message>`
    }
}