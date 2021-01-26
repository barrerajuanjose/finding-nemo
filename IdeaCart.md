# Find your new car
## POC
I'd like to have an API that helps me to buy my new car.
Given a brand and model I want to find it using these conditions,
- The cheapest
- The more expensive
- Less kms
- Newer
- Closer end date

I want to combine these conditions

##THE query is
https://api.mercadolibre.com/sites/MLA/search?limit=50&until=today&category=MLA1744&state=TUxBUENBUGw3M2E1

## Steps
- Site -> Moto/Auto -> (first api call to search) State
- Maybe I first need to look for all the available states in the filter.
- When the state is set, it's easier to the user see all cars.
- User will see price, picture and title.





