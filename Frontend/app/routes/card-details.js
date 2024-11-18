// app/routes/card-details.js
import Route from '@ember/routing/route';

export default class CardDetailsRoute extends Route {
  model(params) {
    return {
      cardId: params.cardId, // Dynamic segment for cardId
      cardName: params.cardName,
      cardBenefits: params.cardBenefits,
    };
  }
}
