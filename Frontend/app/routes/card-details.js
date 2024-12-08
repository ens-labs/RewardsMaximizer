import Route from '@ember/routing/route';

export default class CardDetailsRoute extends Route {
  async model(params) {
    try {
      const response = await fetch(`http://localhost:8080/card/${params.cardId}`);
      if (response.ok) {
        const cardDetails = await response.json();
        return cardDetails;
      } else {
        console.error(`Failed to fetch card details for ID ${params.cardId}`);
        return null;
      }
    } catch (error) {
      console.error('Error fetching card details:', error);
      return null;
    }
  }
}
