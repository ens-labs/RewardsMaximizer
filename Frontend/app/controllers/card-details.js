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
    this.loadCardDetails(); // Fetch card details when the controller is initialized
  }

  loadCardDetails() {
    try {
      this.card = this.model; // Use model data directly
      this.rating = this.model.rating || 0; // Set initial rating
    } catch (error) {
      console.error('Error setting card details:', error);
    }
  }

  @action
  async setRating(newRating) {
    this.rating = newRating;

    try {
      const response = await fetch(`http://localhost:8080/cards/${this.card.cardId}/rating`, {
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
