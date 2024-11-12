import Route from '@ember/routing/route';

export default class WalletRoute extends Route {
  model() {
    // This can be static data for now
    return [
      { cardName: "Chase Freedom Unlimited", cardBenefits: "1.5% cash back on all purchases, 3% on dining, 5% on travel through Chase", iconClass: "fab fa-cc-visa", bgColorClass: "bg-chase" },
      { cardName: "Chase Sapphire Preferred", cardBenefits: "5x points on travel through Chase, 3x on dining, 2x on other travel", iconClass: "fas fa-gem", bgColorClass: "bg-sapphire" },
      { cardName: "Amazon Prime Rewards Visa", cardBenefits: "5% cash back at Amazon and Whole Foods, 2% at restaurants, gas, and transit", iconClass: "fab fa-amazon", bgColorClass: "bg-amazon" }
    ];
  }
}
