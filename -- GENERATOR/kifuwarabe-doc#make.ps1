cls
cd $PSScriptRoot
cd ..
pwd

# Trailing slash.
$product = (Get-Location).Path + "\"
$original = $product + "-- META\kifuwarabe-doc\"

# Trailing NOT slash.
$deployment = $product + "docs\kifuwarabe-doc"

echo "Product: $product"
echo "Original: $original"
echo "Deployment: $deployment"

. "-- META\document.ps1"
Copy-Doc $product $original $deployment

