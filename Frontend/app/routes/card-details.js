import Route from '@ember/routing/route';

export default class CardDetailsRoute extends Route {
  model(params) {
    return {
      cardName: params.cardName,
      cardBenefits: params.cardBenefits,
    };
  }
}
