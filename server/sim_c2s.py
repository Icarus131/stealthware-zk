from flask import Flask, request, jsonify

app = Flask(__name__)

@app.route('/verify', methods=['POST'])
def verify_proof():
    proof_data = request.json

    if proof_data and 'public_inputs' in proof_data:
        return jsonify({"status": "success", "message": "Proof verified successfully"}), 200
    else:
        return jsonify({"status": "error", "message": "Invalid proof data"}), 400

if __name__ == '__main__':
    app.run(port=8080)

