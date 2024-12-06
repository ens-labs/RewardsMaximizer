import Controller from '@ember/controller';
import { action } from '@ember/object';
export default class CardDetailsController extends Controller {
// Get session storage username
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username');
    console.log(this.username);
    sessionStorage.setItem('username', this.username);
  }

  rating = 0; // Store the rating state

  // Assuming cardIcon is coming from the model data
  cardIcon = 'fas fa-credit-card'; // Example: FontAwesome class

  // Set the rating when a user clicks a star
  @action
  setRating(event) {
    const selectedRating = parseInt(event.target.dataset.star, 10); // Get the rating from the data-star attribute
    this.set('rating', selectedRating); // Update the rating state
  }
}
