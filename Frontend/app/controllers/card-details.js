import Controller from '@ember/controller';
import { action } from '@ember/object';
import { tracked } from '@glimmer/tracking';

export default class CardDetailsController extends Controller {
  @tracked rating = 0; // Store the rating state
  @tracked card = {}; // Store the current card details

  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username') || 'Guest';
    sessionStorage.setItem('username', this.username);
  }

  // Lifecycle hook to ensure card data is loaded when the model changes
  @action
  loadCardDetails() {
    if (this.model) {
      this.card = this.model; // This will be set to the model data from the route
      this.rating = this.model.rating || 0; // Initialize the rating if available
    }
  }

  // Update the card's rating
  @action
  async setRating(newRating) {
    this.rating = newRating;

    try {
      const cardId = this.model.cardId; // cardId should match the key in the API endpoint
      const response = await fetch(`http://localhost:8080/cards/${cardId}/rating`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ rating: this.rating }),
      });

      if (response.ok) {
        console.log(`Successfully updated card rating to ${this.rating} stars.`);
      } else {
        console.error('Failed to update card rating.');
      }
    } catch (error) {
      console.error('Network error while updating card rating:', error);
    }
  }
}
