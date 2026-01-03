#!/bin/bash

BASE_URL="http://localhost:3001/api/v1"

echo "🎲 Starting data population for Betting API..."
echo ""

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print success
success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Function to print error
error() {
    echo -e "${RED}✗ $1${NC}"
}

# Function to print info
info() {
    echo -e "${BLUE}→ $1${NC}"
}

# =======================
# 1. CREATE ACCOUNTS
# =======================
info "Creating accounts..."

# Account 1: nimesh
ACCOUNT1=$(curl -s -X POST "$BASE_URL/accounts" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "nimesh",
    "hostname": "host1.betting.local"
  }')

if [ $? -eq 0 ]; then
    ACCOUNT1_ID=$(echo $ACCOUNT1 | grep -o '"id":[0-9]*' | grep -o '[0-9]*' | head -1)
    success "Created account 'nimesh' with ID: $ACCOUNT1_ID"
else
    error "Failed to create account 'nimesh'"
    exit 1
fi

# Account 2: ganga
ACCOUNT2=$(curl -s -X POST "$BASE_URL/accounts" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "ganga",
    "hostname": "host2.betting.local"
  }')

if [ $? -eq 0 ]; then
    ACCOUNT2_ID=$(echo $ACCOUNT2 | grep -o '"id":[0-9]*' | grep -o '[0-9]*' | head -1)
    success "Created account 'ganga' with ID: $ACCOUNT2_ID"
else
    error "Failed to create account 'ganga'"
    exit 1
fi

# Account 3: rajesh
ACCOUNT3=$(curl -s -X POST "$BASE_URL/accounts" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "rajesh",
    "hostname": "host3.betting.local"
  }')

if [ $? -eq 0 ]; then
    ACCOUNT3_ID=$(echo $ACCOUNT3 | grep -o '"id":[0-9]*' | grep -o '[0-9]*' | head -1)
    success "Created account 'rajesh' with ID: $ACCOUNT3_ID"
else
    error "Failed to create account 'rajesh'"
    exit 1
fi

# Account 4: priya
ACCOUNT4=$(curl -s -X POST "$BASE_URL/accounts" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "priya",
    "hostname": "host4.betting.local"
  }')

if [ $? -eq 0 ]; then
    ACCOUNT4_ID=$(echo $ACCOUNT4 | grep -o '"id":[0-9]*' | grep -o '[0-9]*' | head -1)
    success "Created account 'priya' with ID: $ACCOUNT4_ID"
else
    error "Failed to create account 'priya'"
    exit 1
fi

# Account 5: suresh
ACCOUNT5=$(curl -s -X POST "$BASE_URL/accounts" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "suresh",
    "hostname": "host5.betting.local"
  }')

if [ $? -eq 0 ]; then
    ACCOUNT5_ID=$(echo $ACCOUNT5 | grep -o '"id":[0-9]*' | grep -o '[0-9]*' | head -1)
    success "Created account 'suresh' with ID: $ACCOUNT5_ID"
else
    error "Failed to create account 'suresh'"
    exit 1
fi

echo ""

# =======================
# 2. CREATE BATCHES
# =======================
info "Creating batches..."

# ===== ACCOUNT 1 BATCHES =====
# Batch 1 - Race 1 - WIN bets (single horse only)
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT1_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "WIN",
      "race_id": 1
    },
    "bets": [
      {"id": 1, "selection": "1", "stake": 1.0, "cost": 1.0},
      {"id": 2, "selection": "3", "stake": 1.0, "cost": 1.0},
      {"id": 3, "selection": "5", "stake": 0.5, "cost": 0.5}
    ]
  }' > /dev/null && success "Created batch for nimesh - Race 1 (WIN)"

# Batch 2 - Race 2 - PLACE bets (single horse only)
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT1_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "PLACE",
      "race_id": 2
    },
    "bets": [
      {"id": 1, "selection": "2", "stake": 2.0, "cost": 1.0},
      {"id": 2, "selection": "4", "stake": 1.5, "cost": 0.5},
      {"id": 3, "selection": "6", "stake": 1.0, "cost": 0.5}
    ]
  }' > /dev/null && success "Created batch for nimesh - Race 2 (PLACE)"

# Batch 3 - Race 3 - QUINELLA bets (2 horses, any order)
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT1_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "QUINELLA",
      "race_id": 3
    },
    "bets": [
      {"id": 1, "selection": "1/2", "stake": 3.0, "cost": 1.0},
      {"id": 2, "selection": "3/4", "stake": 2.5, "cost": 1.0},
      {"id": 3, "selection": "5/6", "stake": 2.0, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for nimesh - Race 3 (QUINELLA)"

# Batch 4 - Race 4 - EXACTA bets (2 horses, exact order)
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT1_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "EXACTA",
      "race_id": 4
    },
    "bets": [
      {"id": 1, "selection": "2/3", "stake": 1.5, "cost": 0.5},
      {"id": 2, "selection": "7/8", "stake": 1.0, "cost": 0.5}
    ]
  }' > /dev/null && success "Created batch for nimesh - Race 4 (EXACTA)"

# Batch 5 - Race 5 - TRIFECTA bets (3 horses, exact order)
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT1_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "TRIFECTA",
      "race_id": 5
    },
    "bets": [
      {"id": 1, "selection": "1/2/3", "stake": 5.0, "cost": 1.0},
      {"id": 2, "selection": "4/5/6", "stake": 3.0, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for nimesh - Race 5 (TRIFECTA)"

# ===== ACCOUNT 2 BATCHES =====
# Batch 6 - Race 1 - WIN bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT2_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "WIN",
      "race_id": 1
    },
    "bets": [
      {"id": 1, "selection": "2", "stake": 0.5, "cost": 0.5},
      {"id": 2, "selection": "4", "stake": 1.0, "cost": 0.5},
      {"id": 3, "selection": "6", "stake": 0.5, "cost": 0.5},
      {"id": 4, "selection": "8", "stake": 2.0, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for ganga - Race 1 (WIN)"

# Batch 7 - Race 2 - PLACE bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT2_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "PLACE",
      "race_id": 2
    },
    "bets": [
      {"id": 1, "selection": "1", "stake": 2.0, "cost": 1.0},
      {"id": 2, "selection": "3", "stake": 1.5, "cost": 0.5},
      {"id": 3, "selection": "7", "stake": 1.0, "cost": 0.5}
    ]
  }' > /dev/null && success "Created batch for ganga - Race 2 (PLACE)"

# Batch 8 - Race 3 - QUINELLA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT2_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "QUINELLA",
      "race_id": 3
    },
    "bets": [
      {"id": 1, "selection": "1/3", "stake": 2.5, "cost": 1.0},
      {"id": 2, "selection": "2/5", "stake": 2.0, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for ganga - Race 3 (QUINELLA)"

# Batch 9 - Race 4 - TRIFECTA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT2_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "TRIFECTA",
      "race_id": 4
    },
    "bets": [
      {"id": 1, "selection": "1/2/3", "stake": 5.0, "cost": 1.0},
      {"id": 2, "selection": "4/5/6", "stake": 3.0, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for ganga - Race 4 (TRIFECTA)"

# ===== ACCOUNT 3 BATCHES =====
# Batch 10 - Race 1 - QUINELLA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT3_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "QUINELLA",
      "race_id": 1
    },
    "bets": [
      {"id": 1, "selection": "3/4", "stake": 2.5, "cost": 1.0},
      {"id": 2, "selection": "5/7", "stake": 2.0, "cost": 1.0},
      {"id": 3, "selection": "8/9", "stake": 1.5, "cost": 0.5}
    ]
  }' > /dev/null && success "Created batch for rajesh - Race 1 (QUINELLA)"

# Batch 11 - Race 2 - WIN bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT3_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "WIN",
      "race_id": 2
    },
    "bets": [
      {"id": 1, "selection": "1", "stake": 4.0, "cost": 1.0},
      {"id": 2, "selection": "5", "stake": 3.0, "cost": 1.0},
      {"id": 3, "selection": "7", "stake": 2.0, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for rajesh - Race 2 (WIN)"

# Batch 12 - Race 3 - EXACTA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT3_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "EXACTA",
      "race_id": 3
    },
    "bets": [
      {"id": 1, "selection": "2/3", "stake": 2.0, "cost": 1.0},
      {"id": 2, "selection": "4/5", "stake": 1.5, "cost": 0.5},
      {"id": 3, "selection": "6/7", "stake": 1.0, "cost": 0.5}
    ]
  }' > /dev/null && success "Created batch for rajesh - Race 3 (EXACTA)"

# Batch 13 - Race 5 - TRIFECTA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT3_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "TRIFECTA",
      "race_id": 5
    },
    "bets": [
      {"id": 1, "selection": "2/4/6", "stake": 8.0, "cost": 2.0},
      {"id": 2, "selection": "1/3/5", "stake": 6.0, "cost": 2.0}
    ]
  }' > /dev/null && success "Created batch for rajesh - Race 5 (TRIFECTA)"

# ===== ACCOUNT 4 BATCHES =====
# Batch 14 - Race 2 - WIN bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT4_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "WIN",
      "race_id": 2
    },
    "bets": [
      {"id": 1, "selection": "1", "stake": 5.0, "cost": 2.0},
      {"id": 2, "selection": "3", "stake": 4.0, "cost": 2.0},
      {"id": 3, "selection": "5", "stake": 3.0, "cost": 1.0},
      {"id": 4, "selection": "9", "stake": 2.0, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for priya - Race 2 (WIN)"

# Batch 15 - Race 3 - PLACE bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT4_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "PLACE",
      "race_id": 3
    },
    "bets": [
      {"id": 1, "selection": "2", "stake": 3.5, "cost": 1.0},
      {"id": 2, "selection": "7", "stake": 2.5, "cost": 1.0},
      {"id": 3, "selection": "4", "stake": 2.0, "cost": 0.5}
    ]
  }' > /dev/null && success "Created batch for priya - Race 3 (PLACE)"

# Batch 16 - Race 4 - QUINELLA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT4_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "QUINELLA",
      "race_id": 4
    },
    "bets": [
      {"id": 1, "selection": "1/3", "stake": 4.0, "cost": 2.0},
      {"id": 2, "selection": "5/8", "stake": 3.0, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for priya - Race 4 (QUINELLA)"

# Batch 17 - Race 5 - EXACTA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT4_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "EXACTA",
      "race_id": 5
    },
    "bets": [
      {"id": 1, "selection": "3/5", "stake": 3.0, "cost": 1.5},
      {"id": 2, "selection": "7/9", "stake": 2.5, "cost": 1.0}
    ]
  }' > /dev/null && success "Created batch for priya - Race 5 (EXACTA)"

# ===== ACCOUNT 5 BATCHES =====
# Batch 18 - Race 1 - EXACTA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT5_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "EXACTA",
      "race_id": 1
    },
    "bets": [
      {"id": 1, "selection": "1/2", "stake": 6.0, "cost": 2.0},
      {"id": 2, "selection": "4/5", "stake": 4.5, "cost": 1.5}
    ]
  }' > /dev/null && success "Created batch for suresh - Race 1 (EXACTA)"

# Batch 19 - Race 2 - TRIFECTA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT5_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "TRIFECTA",
      "race_id": 2
    },
    "bets": [
      {"id": 1, "selection": "2/3/4", "stake": 10.0, "cost": 2.0},
      {"id": 2, "selection": "5/6/7", "stake": 8.0, "cost": 2.0}
    ]
  }' > /dev/null && success "Created batch for suresh - Race 2 (TRIFECTA)"

# Batch 20 - Race 3 - WIN bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT5_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "WIN",
      "race_id": 3
    },
    "bets": [
      {"id": 1, "selection": "1", "stake": 10.0, "cost": 5.0},
      {"id": 2, "selection": "3", "stake": 8.0, "cost": 4.0},
      {"id": 3, "selection": "5", "stake": 6.0, "cost": 3.0},
      {"id": 4, "selection": "7", "stake": 4.0, "cost": 2.0}
    ]
  }' > /dev/null && success "Created batch for suresh - Race 3 (WIN)"

# Batch 21 - Race 4 - PLACE bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT5_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "PLACE",
      "race_id": 4
    },
    "bets": [
      {"id": 1, "selection": "2", "stake": 5.0, "cost": 2.5},
      {"id": 2, "selection": "6", "stake": 4.0, "cost": 2.0},
      {"id": 3, "selection": "9", "stake": 3.0, "cost": 1.5}
    ]
  }' > /dev/null && success "Created batch for suresh - Race 4 (PLACE)"

# Batch 22 - Race 5 - QUINELLA bets
curl -s -X POST "$BASE_URL/accounts/$ACCOUNT5_ID/batches" \
  -H "Content-Type: application/json" \
  -d '{
    "meta": {
      "bet_type": "QUINELLA",
      "race_id": 5
    },
    "bets": [
      {"id": 1, "selection": "2/4", "stake": 7.0, "cost": 3.0},
      {"id": 2, "selection": "6/8", "stake": 5.0, "cost": 2.0}
    ]
  }' > /dev/null && success "Created batch for suresh - Race 5 (QUINELLA)"

echo ""
success "✅ Data population complete!"
echo ""
info "Summary:"
echo "  - Accounts created: 5"
echo "  - Batches created: 22"
echo "  - Total bets: ~60+"
echo ""
info "Bet types with correct formats:"
echo "  - WIN: Single horse (e.g., '1', '3', '5')"
echo "  - PLACE: Single horse (e.g., '2', '4', '6')"
echo "  - QUINELLA: Two horses any order (e.g., '1/2', '3/4')"
echo "  - EXACTA: Two horses exact order (e.g., '2/3', '7/8')"
echo "  - TRIFECTA: Three horses exact order (e.g., '1/2/3', '4/5/6')"
